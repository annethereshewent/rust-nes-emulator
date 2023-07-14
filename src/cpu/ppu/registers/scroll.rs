pub struct ScrollRegister {
  v: u16,
  t: u16,
  x: u8,
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

  pub fn copy_y(&mut self) {
    // per https://www.nesdev.org/wiki/PPU_scrolling#Tile_and_attribute_fetching
    // we copy the following bits from t to v:
    // v: GHIA.BC DEF..... <- t: GHIA.BC DEF.....
    let fine_coarse_mask: u16 = 0b111_10_11111_00000;

    self.v = (self.v & !fine_coarse_mask) | (self.t & fine_coarse_mask);
  }

  pub fn copy_x(&mut self) {
    // v: ....A.. ...BCDEF <- t: ....A.. ...BCDEF
    let coarse_x_mask = 0b1_00000_11111;

    // reset coarse x in v first, then copy it over from t
    self.v = (self.v & !(coarse_x_mask)) | (self.t & coarse_x_mask);
  }

  pub fn fine_x(&self) -> u8 {
    self.x
  }

  pub fn fine_y(&self) -> u16 {
    (self.v >> 12) & 0b111
  }

  // per https://github.com/lukexor/tetanes/blob/main/src/ppu/scroll.rs
  pub fn attribute_shift(&self) -> u16 {
    (self.v & 0x02) | ((self.v >> 4) & 0x04)
  }

  // see https://www.nesdev.org/wiki/PPU_scrolling#Tile_and_attribute_fetching
  pub fn tile_address(&self) -> u16 {
      0x2000 | (self.v & 0xfff)
  }

  pub fn attribute_address(&self) -> u16 {
    // base  nametable select    y bits                   x bits
    0x23C0 | (self.v & 0x0C00) | ((self.v >> 4) & 0x38) | ((self.v >> 2) & 0x07)
  }

  pub fn set_scroll(&mut self, val: u8) {
    if !self.latch {
      self.x = val & 0b111;
      let coarse_x = val >> 3;

      // clear coarse x in t
      self.t &= !(0b11111);

      self.t |= coarse_x as u16;
    } else {
      let fine_y = val & 0b111;
      let coarse_y = val >> 3;

      self.t &= !(0b111_00_11111_00000);

      self.t |= (coarse_y as u16) << 5;
      self.t |= (fine_y as u16) << 12;
    }

    self.latch = !self.latch;
  }

  pub fn set_nametable_select(&mut self, val: u8) {
    let nametable_select = val & 0b11;

    self.t = (self.t & !(0b000_11_00000_00000)) | ((nametable_select as u16) << 10);
  }

  pub fn set_address(&mut self, val: u8) {
    if !self.latch {
      // write to high address
      self.t = (self.t & 0xff) | ((val as u16 & 0b111111) << 8);
    } else {
      // write to low address
      let high_bits = self.t >> 8;
      self.t = (high_bits << 8) | val as u16;

      self.v = self.t & 0x3fff;
    }

    self.latch = !self.latch
  }

  pub fn get_address(&self) -> u16 {
    self.v
  }

  // per https://www.nesdev.org/wiki/PPU_scrolling#Wrapping_around
  pub fn increment_x(&mut self) {
    // coarse x is 31, wrap around
    if (self.v & 0b11111) == 31 {
      self.v &= !(0b11111); // clear coarse x
      self.v ^= 0x400; // switch nametable
    } else {
      self.v += 1;
    }
  }

  // see above
  pub fn increment_y(&mut self) {
    // if fine y is less than 7
    if (self.v & 0x7000) != 0x7000 {
      self.v += 0x1000; // increment fine y
    } else {
      self.v &= !(0x7000);
      let mut y = (self.v & 0x3e0) >> 5; // let y = coarse y
      if y == 29 {
        y = 0;
        self.v ^= 0x800;
      } else if y == 31 {
        y = 0
      } else {
        y += 1;
      }

      self.v = (self.v & !(0x3e0)) | y << 5;
    }
  }

  pub fn increment_address(&mut self, increment: u8) {
    self.v = (self.v + increment as u16) & 0x3fff;
  }

  pub fn reset_latch(&mut self) {
    self.latch = false
  }
}