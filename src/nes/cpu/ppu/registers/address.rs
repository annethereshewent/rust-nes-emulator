pub struct AddressRegister {
  pub latch: bool,
  pub higher: u8,
  pub lower: u8
}

impl AddressRegister {
  pub fn new() -> Self {
    AddressRegister { latch: false, higher: 0, lower: 0 }
  }

  fn set(&mut self, val: u16) {
    self.lower = (val & 0xff) as u8;
    self.higher = (val >> 8) as u8;
  }

  pub fn update(&mut self, val: u8) {
    if !self.latch {
      self.higher = val
    } else {
      self.lower = val
    }

    // for mirroring
    if self.get() > 0x3fff {
      self.set(self.get() & 0x3fff);
    }

    self.latch = !self.latch;
  }

  pub fn increment(&mut self, val: u8) {
    let (result, carry) = self.lower.overflowing_add(val);

    self.lower = result;
    if carry {
      self.higher = self.higher.wrapping_add(1);
    }

    // for mirroring
    if self.get() > 0x3fff {
      self.set(self.get() & 0x3fff);
    }
  }

  pub fn get(&self) -> u16 {
    (self.higher as u16) << 8 | (self.lower as u16)
  }
}