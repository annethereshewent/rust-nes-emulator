pub struct ScrollRegister {
  pub x: u8,
  pub y: u8,
  pub latch: bool
}

impl ScrollRegister {
  pub fn new() -> Self {
    ScrollRegister {
      x: 0,
      y: 0,
      latch: false
    }
  }

  pub fn set(&mut self, val: u8) {
    if !self.latch {
      self.x = val;
    } else {
      self.y = val;
    }

    self.latch = !self.latch;
  }

  pub fn reset_latch(&mut self) {
    self.latch = false
  }
}