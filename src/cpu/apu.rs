use self::{pulse::Pulse, triangle::Triangle, noise::Noise, dmc::DMC};

pub mod pulse;
pub mod triangle;
pub mod noise;
pub mod dmc;
pub mod registers;
pub mod frame_counter;

pub struct APU {
  pub pulse1: Pulse,
  pub pulse2: Pulse,
  pub triangle: Triangle,
  pub noise: Noise,
  pub dmc: DMC,
  cycles: usize,
  half_cycle: bool
}

impl APU {
  pub fn new() -> Self {
    Self {
      pulse1: Pulse::new(),
      pulse2: Pulse::new(),
      triangle: Triangle::new(),
      noise: Noise::new(),
      dmc: DMC::new(),
      cycles: 0,
      half_cycle: false
    }
  }

  pub fn tick(&mut self, cycles: u16) {
    let half_cycles = cycles / 2;
    if cycles == 1 {
      if self.half_cycle {
        self.cycles += 1;

        self.pulse1.tick(1);
        self.pulse2.tick(1);
        self.noise.tick(1);
      }
      self.half_cycle = !self.half_cycle;
    } else {
      self.cycles += half_cycles as usize;

      self.pulse1.tick(half_cycles);
      self.pulse2.tick(half_cycles);
      self.noise.tick(half_cycles);
    }

    self.triangle.tick(cycles);
    self.clock_frame_counter(cycles);

  }

  fn clock_frame_counter(&mut self, cycles: u16) {

  }
}