use self::{pulse::{Pulse, PulseChannel}, triangle::Triangle, noise::Noise, dmc::DMC, frame_counter::{FrameCounter, FrameCounterMode}, registers::status::Status};

pub mod pulse;
pub mod triangle;
pub mod noise;
pub mod dmc;
pub mod registers;
pub mod frame_counter;
pub mod envelope;

const CYCLES_PER_SAMPLE: usize = 1790000 / 44100;

pub struct APU {
  pub pulse1: Pulse,
  pub pulse2: Pulse,
  pub triangle: Triangle,
  pub noise: Noise,
  pub dmc: DMC,
  pub frame_counter: FrameCounter,
  pub status: Status,
  pub audio_samples: [f32; 4096],
  pub buffer_index: usize,
  pub previous_value: f32,
  pub irq_pending: bool,
  cycles: usize,
  half_cycle: u8,
  irq_inhibit: bool,
  pulse_table: [f32; 31],
  tnd_table: [f32; 203]
}

pub const LENGTH_TABLE: [u8; 32] = [
  10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96,
  22, 192, 24, 72, 26, 16, 28, 32, 30,
];

impl APU {
  pub fn new() -> Self {
    let mut pulse_table = [0.0; 31];
    for (i, val) in pulse_table.iter_mut().enumerate().skip(1) {
        *val = 95.52 / (8_128.0 / (i as f32) + 100.0);
    }

    let mut tnd_table = [0.0; 203];
    for (i, val) in tnd_table.iter_mut().enumerate().skip(1) {
        *val = 163.67 / (24_329.0 / (i as f32) + 100.0);
    }

    Self {
      pulse1: Pulse::new(PulseChannel::One),
      pulse2: Pulse::new(PulseChannel::Two),
      triangle: Triangle::new(),
      noise: Noise::new(),
      dmc: DMC::new(),
      cycles: 0,
      half_cycle: 0,
      frame_counter: FrameCounter::new(),
      irq_inhibit: false,
      irq_pending: false,
      status: Status::from_bits_truncate(0b0),
      pulse_table,
      tnd_table,
      audio_samples: [0.0; 4096],
      previous_value: 0.0,
      buffer_index: 0
    }
  }

  pub fn tick(&mut self, cycles: u16) {
    self.dmc.check_dma_status(cycles as i8);

    let remainder = cycles % 2;
    let halved_cycles = cycles / 2;

    if remainder == 1 {
      self.half_cycle += 1;
    }

    // once there have been two "half cycles" added, tick the channels by 1
    if self.half_cycle == 2 {
      self.pulse1.tick(1);
      self.pulse2.tick(1);
      self.noise.tick(1);
      self.dmc.tick(1);

      self.half_cycle = 0;
    }

    if halved_cycles != 0 {
      self.pulse1.tick(halved_cycles);
      self.pulse2.tick(halved_cycles);
      self.noise.tick(halved_cycles);
      self.dmc.tick(halved_cycles);
    }

    // triangle ticks at same rate as CPU
    self.triangle.tick(cycles);
    self.clock_frame_counter(cycles);

    self.cycles += cycles as usize;

    if self.cycles >= CYCLES_PER_SAMPLE {
      self.sample_audio();
      self.cycles -= CYCLES_PER_SAMPLE;
    }
  }

  pub fn sample_audio(&mut self) {
    let audio_sample = self.get_sample();

    if self.buffer_index < 4096 {
      self.audio_samples[self.buffer_index] = audio_sample;
      self.buffer_index += 1;
    }
  }

  fn clock_frame_counter(&mut self, cycles: u16) {
    let step = self.frame_counter.clock(cycles);

    if matches!(self.frame_counter.mode, FrameCounterMode::Step4) && !self.irq_inhibit && self.frame_counter.step == 3 {
      self.irq_pending = true;
    }

    // see https://www.nesdev.org/wiki/APU_Frame_Counter
    match step {
      1 | 3 => self.clock_quarter_frame(),
      2 | 5 => {
        self.clock_quarter_frame();
        self.clock_half_frame();
      },
      _ => ()
    }

    // if write to frame counter set it to step 5 mode, then clock immediately
    if self.frame_counter.poll(cycles) && matches!(self.frame_counter.mode, FrameCounterMode::Step5) {
      self.clock_quarter_frame();
      self.clock_half_frame();
    }
  }

  fn clock_quarter_frame(&mut self) {
    self.pulse1.clock_quarter_frame();
    self.pulse2.clock_quarter_frame();
    self.triangle.clock_quarter_frame();
    self.noise.clock_quarter_frame();
  }

  fn clock_half_frame(&mut self) {
    self.pulse1.clock_half_frame();
    self.pulse2.clock_half_frame();
    self.triangle.clock_half_frame();
    self.noise.clock_half_frame();
  }

  pub fn write_status(&mut self, val: u8) {
    self.status = Status::from_bits_truncate(val);

    self.toggle_channels();
  }

  pub fn toggle_channels(&mut self) {
    self.dmc.toggle(self.status.contains(Status::DMC_ENABLE), self.cycles);
    self.noise.toggle(self.status.contains(Status::NOISE_ENABLE));
    self.pulse1.toggle(self.status.contains(Status::PULSE1_ENABLE));
    self.pulse2.toggle(self.status.contains(Status::PULSE2_ENABLE));
    self.triangle.toggle(self.status.contains(Status::TRIANGLE_ENABLE));
  }

  pub fn read_status(&mut self) -> u8 {
    self.irq_pending = false;
    self.status.bits()
  }

  pub fn get_sample(&self) -> f32 {
    let pulse_index = (self.pulse1.output() + self.pulse2.output()) as usize % self.pulse_table.len();

    let triangle_out = self.triangle.output();
    let noise_out = 0.0;
    let dmc_out = self.dmc.output();

    println!("{}", dmc_out);

    let tnd_index = (3.0 * triangle_out + ((2.0 * noise_out) + dmc_out)) as usize;

    self.pulse_table[pulse_index] + self.tnd_table[tnd_index]
    // self.pulse_table[pulse_index]
  }

  pub fn write_frame_counter(&mut self, val: u8) {
    self.frame_counter.write(val, self.cycles);

    self.irq_inhibit = (val >> 6) & 0b1 == 1;

    if self.irq_inhibit {
      self.irq_pending = false;
    }

  }
}