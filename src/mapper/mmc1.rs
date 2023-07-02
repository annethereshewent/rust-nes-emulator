use crate::cartridge::{Cartridge, Mirroring};

use super::{MemMap, MappedRead};

const PRG_RAM_SIZE: usize = 32_768;
const CHR_RAM_SIZE: usize = 8192;


struct Mmc1Registers {
  write_occurred: u8,
  shift: u8,
  control: u8,
  chr0: u8,
  chr1: u8,
  prg: u8
}
pub struct Mmc1 {
  registers: Mmc1Registers,
  submapper_num: u8,
  mirroring: Mirroring,
  chr_select: bool,
  chr_banks: MemoryBanks,
  prg_ram_banks: MemoryBanks,
  prg_rom_banks: MemoryBanks
}

impl Mmc1 {
  pub fn load(cartridge: &mut Cartridge) -> Self {
    cartridge.chr_ram.resize(CHR_RAM_SIZE, 0);
    cartridge.prg_ram.resize(PRG_RAM_SIZE, 0);

    Self {
      registers: Mmc1Registers {
        write_occurred: 0,
        shift: 0x10,
        control: 0xc,
        chr0: 0,
        chr1: 0,
        prg: 0
      },
      submapper_num: 0,
      mirroring: Mirroring::SingleScreenA,
      chr_select: cartridge.prg_rom.len() == 0x80000,
      chr_banks: MemoryBanks {},
      prg_ram_banks: MemoryBanks {},
      prg_rom_banks: MemoryBanks {}
    }
  }
}

impl MemMap for Mmc1 {
  fn map_read(&mut self, address: u16) -> MappedRead {
    MappedRead::None
  }
}

pub struct MemoryBanks {

}

impl MemoryBanks {

}