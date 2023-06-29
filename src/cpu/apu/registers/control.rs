pub struct Control {
  val: u8
}

impl Control {
  pub fn new() -> Self {
    Self {
      val: 0
    }
  }
  pub fn duty_cycle(&self) -> u8 {
    self.val >> 6
  }

  pub fn length_counter_halt(&self) -> u8 {
    (self.val >> 5) & 0b1
  }

  pub fn constant_vol_env_flag(&self) -> u8 {
    (self.val >> 4) & 0b1
  }

  pub fn envelope_divider_period(&self) -> u8 {
    self.val & 0b1111
  }

  pub fn set(&mut self, val: u8) {
    self.val = val;
  }
}