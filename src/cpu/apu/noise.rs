use super::{envelope::Envelope, registers::noise_control::NoiseControlRegister, LENGTH_TABLE};

const FREQUENCY_TABLE: [u16; 16] = [
  4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068
];

const SHIFT_BIT_MASK: u16 = !0x8000;

enum ShiftMode {
  Zero,
  One
}

pub struct Noise {
  enabled: bool,
  frequency_counter: i16,
  frequency_timer: u16,
  shift: u16,
  length_counter: u8,
  envelope: Envelope,
  shift_mode: ShiftMode,
  pub control: NoiseControlRegister
}

impl Noise {
  pub fn new() -> Self {
    Self {
      enabled: false,
      frequency_counter: 0,
      frequency_timer: 0,
      shift: 0,
      length_counter: 0,
      envelope: Envelope::new(),
      shift_mode: ShiftMode::Zero,
      control: NoiseControlRegister::new()
    }
  }

  pub fn tick(&mut self, cycles: u16) {
    if self.frequency_counter > 0 {
      self.frequency_counter -= cycles as i16;
    } else {
      self.frequency_counter += self.frequency_timer as i16;

      let shift_amount = if matches!(self.shift_mode, ShiftMode::Zero) { 1 } else { 6 };

      let bit1 = self.shift & 1;
      let bit2 = (self.shift >> shift_amount) & 0b1;

      self.shift = (self.shift & SHIFT_BIT_MASK) | (bit1 ^ bit2) << 14;
      self.shift >>= 1;
    }
  }

  pub fn output(&self) -> f32 {
    if self.shift & 1 == 0 && self.length_counter != 0 {
      if self.control.constant_volume() == 0 {
        self.envelope.volume as f32
      } else {
        self.control.envelope_volume() as f32
      }
    } else {
      0.0
    }
  }

  pub fn toggle(&mut self, enabled: bool) {
    self.enabled = enabled;
    if !enabled {
      self.length_counter = 0;
    }
  }

  pub fn write_length(&mut self, val: u8) {
    if self.enabled {
      self.length_counter = LENGTH_TABLE[(val > 3) as usize];
    }
  }

  pub fn write_timer(&mut self, val: u8) {
    let noise_period = val & 0b1111;

    let loop_noise = (val >> 7) & 0b1;

    self.shift_mode = if loop_noise == 0 { ShiftMode::Zero } else { ShiftMode::One };

    self.frequency_timer = FREQUENCY_TABLE[noise_period as usize] - 1;
  }

  pub fn clock_quarter_frame(&mut self) {
    self.envelope.clock(self.control.envelope_volume(), self.control.length_counter_halt())
  }

  pub fn clock_half_frame(&mut self) {
    self.clock_length_counter();
  }

  fn clock_length_counter(&mut self) {
    if self.control.length_counter_halt() == 0 && self.length_counter > 0 {
      self.length_counter -= 1;
    }
  }
}