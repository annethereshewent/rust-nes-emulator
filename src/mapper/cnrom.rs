use crate::cartridge::Cartridge;

use super::MapperActions;

const CHR_BANK_SIZE: usize = 8192;

pub struct Cnrom {
  chr_bank: usize,
  chr_page_size: usize
}

impl Cnrom {
  pub fn load(cartridge: &mut Cartridge) -> Self {
    Self {
      chr_bank: 0,
      chr_page_size:  cartridge.chr_rom.len() / CHR_BANK_SIZE
    }
  }
}

impl MapperActions for Cnrom {
  fn mem_write(&mut self, address: u16, val: u8) -> Option<usize> {
    match address {
      0x8000..=0xffff => {
        self.chr_bank = (val as usize % self.chr_page_size) * CHR_BANK_SIZE;
      },
      _ => ()
    }
    None
  }
  fn mem_read(&mut self, address: u16) -> Option<usize> {
    match address {
      0x0000..=0x1fff => {
        let page = self.chr_bank;

        Some(page | (address as usize) & (CHR_BANK_SIZE - 1))
      }
      _ => None
    }
  }
}