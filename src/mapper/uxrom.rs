use crate::cartridge::{Mirroring, Cartridge};

use super::MapperActions;

const PRG_ROM_BANK_SIZE: usize = 16_384;
const CHR_RAM_SIZE: usize = 8192;

pub struct Uxrom {
  prg_rom_banks: [usize; 2],
  prg_rom_page_size: u8,
  mirroring: Mirroring
}

impl Uxrom {

  pub fn load(cartridge: &mut Cartridge) -> Self {

    cartridge.chr_ram.resize(CHR_RAM_SIZE, 0);

    let mut prg_rom_page_size: u8 = (cartridge.prg_rom.len() / PRG_ROM_BANK_SIZE) as u8;

    if prg_rom_page_size == 0 {
      prg_rom_page_size = 1;
    }

    let mut prg_rom_banks: [usize; 2] = [0; 2];

    let last_bank = prg_rom_page_size - 1;

    prg_rom_banks[1] = (last_bank as usize) * PRG_ROM_BANK_SIZE;

    Self {
      mirroring: cartridge.mirroring,
      prg_rom_page_size,
      prg_rom_banks
    }
  }

  fn get_bank_address(&self, bank: u8) -> usize {
    ((bank % self.prg_rom_page_size) as usize) * PRG_ROM_BANK_SIZE
  }

  pub fn translate_address(&self, address: u16) -> Option<usize> {
    let page = self.prg_rom_banks[self.get_bank_number(address)];

    Some(page | (address as usize) & (PRG_ROM_BANK_SIZE - 1))
  }

  pub fn get_bank_number(&self, address: u16) -> usize {
    match address {
      0x8000..=0xbfff => {
        0
      }
      0xc000..=0xffff => {
        1
      }
      _ => panic!("not possible")
    }
  }
}

impl MapperActions for Uxrom {
  fn mem_read(&mut self, address: u16) -> Option<usize> {
    match address {
      0x0000..=0x1fff => Some(address as usize),
      0x6000..=0x7fff => None,
      0x8000..=0xffff => self.translate_address(address),
      _ => panic!("not possible")
    }
  }

  fn mem_write(&mut self, address: u16, value: u8) -> Option<usize> {
    match address {
      0x8000..=0xffff => {
        self.prg_rom_banks[0] = self.get_bank_address(value);
        None
      }
      _ => panic!("can't be")
    }
  }

  fn mirroring(&self) -> Mirroring {
    self.mirroring
  }
 }