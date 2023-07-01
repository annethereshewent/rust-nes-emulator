pub struct FlagsRateRegister {
  val: u8
}

impl FlagsRateRegister {
  pub fn new() -> Self {
    Self {
      val: 0
    }
  }

  pub fn set(&mut self, val: u8) {
    self.val = val;
  }

  pub fn irq_enabled(&self) -> bool {
    self.val >> 7 == 1
  }

  pub fn loop_flag(&self) -> bool {
    self.val >> 6 & 0b1 == 1
  }

  pub fn rate_index(&self) -> u8 {
    self.val & 0b1111
  }
}