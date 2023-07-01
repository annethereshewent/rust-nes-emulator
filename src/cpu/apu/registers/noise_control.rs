pub struct NoiseControlRegister {
  val: u8
}

impl NoiseControlRegister {
  pub fn new() -> Self {
    Self {
      val: 0
    }
  }

  pub fn set(&mut self, val: u8) {
    self.val = val;
  }

  pub fn envelope_volume(&self) -> u8 {
    self.val & 0b1111
  }

  pub fn constant_volume(&self) -> u8 {
    (self.val >> 4) & 0b1
  }

  pub fn length_counter_halt(&self) -> u8 {
    (self.val >> 5) & 0b1
  }
}