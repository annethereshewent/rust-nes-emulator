pub struct Sweep {
  val: u8,
  pub reload: bool,
  pub counter: u8,
}

impl Sweep {
  pub fn new() -> Self {
    Self {
      val: 0,
      reload: false,
      counter: 0
    }
  }
  pub fn enabled(&self) -> bool {
    self.val >> 7 == 1
  }

  pub fn divider_period(&self) -> u8 {
    self.val >> 4 & 0b111
  }

  pub fn negate_flag(&self) -> bool {
    (self.val >> 3) & 0b1 == 1
  }

  pub fn shift_count(&self) -> u8 {
    self.val & 0b111
  }

  pub fn set(&mut self, val: u8) {
    self.val = val;
    self.reload = true;
  }
}