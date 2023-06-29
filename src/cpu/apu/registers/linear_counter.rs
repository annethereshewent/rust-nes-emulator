pub struct LinearCounter {
  val: u8
}

impl LinearCounter {
  pub fn new() -> Self {
    Self {
      val: 0
    }
  }

  pub fn control_flag(&self) -> u8 {
    self.val >> 7
  }

  pub fn counter_reload(&self) -> u8 {
    self.val & 0b111111
  }

  pub fn set(&mut self, val: u8) {
    self.val = val;
  }
}