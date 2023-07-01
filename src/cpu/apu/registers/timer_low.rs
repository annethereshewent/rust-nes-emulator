pub struct TimerLow {
  val: u8
}

impl TimerLow {
  pub fn new() -> Self {
    Self{
      val: 0
    }
  }

  pub fn set(&mut self, val: u8) {
    self.val = val;
  }

  pub fn bits(&self) -> u8 {
    self.val
  }
}

