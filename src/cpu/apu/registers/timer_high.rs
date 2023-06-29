pub struct TimerHigh {
  val: u8
}

impl TimerHigh {
  pub fn new() -> Self {
    Self {
      val: 0
    }
  }

  pub fn timer_bits(&self) -> u8 {
    self.val & 0b111
  }

  pub fn length_counter_load(&self) -> u8 {
    self.val >> 3
  }

  pub fn set(&mut self, val: u8) {
    self.val = val;
  }
}