bitflags! {
  pub struct ControlRegister: u8 {
    const BASE_NAMETABLE0               = 0b1;
    const BASE_NAMETABLE1               = 0b10;
    const VRAM_ADDRESS_INCREMENT        = 0b100;
    const SPRITE_PATTERN_TABLE_ADDR     = 0b1000;
    const BACKGROUND_PATTERN_TABLE_ADDR = 0b10000;
    const SPRITE_SIZE                   = 0b100000;
    const PPU_MASTER_SLAVE_SELECT       = 0b1000000;
    const GENERATE_NMI_INTERRUPT        = 0b10000000;
  }
}

impl ControlRegister {
  pub fn base_table_address(&self) -> u16 {
    let base_table_value = self.bits() & 0b11;

    match base_table_value {
      0 => 0x2000,
      1 => 0x2400,
      2 => 0x2800,
      3 => 0x2c00,
      _ => panic!("should never get here")
    }
  }

  pub fn vram_address_increment(&self) -> u8 {
    if self.contains(ControlRegister::VRAM_ADDRESS_INCREMENT) { 32 } else { 1 }
  }

  pub fn sprite_pattern_table_address(&self) -> u16 {
    if self.contains(ControlRegister::SPRITE_PATTERN_TABLE_ADDR) { 0x1000 } else { 0 }
  }

  pub fn background_pattern_table_addr(&self) -> u16 {
    if self.contains(ControlRegister::BACKGROUND_PATTERN_TABLE_ADDR) { 0x1000 } else { 0}
  }

  pub fn sprite_size(&self) -> u8 {
    if self.contains(ControlRegister::SPRITE_SIZE) { 16 } else { 8 }
  }

  pub fn master_slave_select(&self) -> u8 {
    if self.contains(ControlRegister::PPU_MASTER_SLAVE_SELECT) { 1 } else { 0 }
  }

  pub fn generate_nmi_interrupt(&self) -> bool {
    self.contains(ControlRegister::GENERATE_NMI_INTERRUPT)
  }
}