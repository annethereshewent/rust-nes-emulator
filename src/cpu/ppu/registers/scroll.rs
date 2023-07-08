pub struct ScrollRegister {
  pub v: u16,
  pub t: u16,
  pub x: u8,
  pub latch: bool
}

impl ScrollRegister {
  pub fn new() -> Self {
    ScrollRegister {
      x: 0,
      v: 0,
      t: 0,
      latch: false
    }
  }

  pub fn set_scroll(&mut self, val: u8) {
    if !self.latch {
      self.x = val & 0b111;
      let coarse_x = val >> 3;
      self.t |= coarse_x as u16;
    } else {
      let fine_y = val & 0b111;
      let coarse_y = val >> 3;

      self.t |= (coarse_y as u16) << 5;
      self.t |= (fine_y as u16) << 12;
    }

    self.latch = !self.latch;
  }

  pub fn set_nametable_select(&mut self, val: u8) {
    let nametable_select = val & 0b11;

    self.t |= (nametable_select as u16) << 10;
  }

  pub fn set_address(&mut self, val: u8) {
    if !self.latch {
      self.t |= (val as u16 & 0b111111) << 8;

      self.t &= !(1 << 14)
    } else {
      self.t |= val as u16;
      self.v = self.t;
    }

    self.latch = !self.latch
  }

  pub fn get_address(&self) -> u16 {
    self.v
  }

  pub fn increment_x(&mut self) {

  }

  pub fn increment_y(&mut self) {

  }

  pub fn increment_address(&mut self, increment: u8) {
    self.v += increment as u16;
  }

  pub fn reset_latch(&mut self) {
    self.latch = false
  }
}