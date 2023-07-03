use crate::cartridge::{Cartridge, Mirroring};

use super::MapperActions;

const PRG_RAM_SIZE: usize = 32_768;
const CHR_RAM_SIZE: usize = 8192;

const CHR_WINDOW: usize = 4096;
const PRG_RAM_WINDOW: usize = 8192;
const PRG_ROM_WINDOW: usize = 16_384;


struct Mmc1Registers {
  write_occurred: u8,
  shift: u8,
  control: u8,
  chr0: u8,
  chr1: u8,
  prg: u8,
  current_shift_write: u8
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


    let chr_capacity = if !cartridge.chr_rom.is_empty() { cartridge.chr_rom.len() } else { cartridge.chr_ram.len() };
    let mut mmc1 = Self {
      registers: Mmc1Registers {
        write_occurred: 0,
        shift: 0x10,
        control: 0xc,
        chr0: 0,
        chr1: 0,
        prg: 0,
        current_shift_write: 0
      },
      submapper_num: 0,
      mirroring: Mirroring::SingleScreenA,
      chr_select: cartridge.prg_rom.len() == 0x80000,
      chr_banks: MemoryBanks::new(0, 0x1fff, chr_capacity, CHR_WINDOW),
      prg_ram_banks: MemoryBanks::new(0x6000, 0x7fff, cartridge.prg_ram.len(), PRG_RAM_WINDOW),
      prg_rom_banks: MemoryBanks::new(0x8000, 0xffff, cartridge.prg_rom.len(), PRG_ROM_WINDOW),
    };


    mmc1.update_banks(0);

    mmc1
  }

  fn reset_shift_register(&mut self) {
    self.registers.shift = 0;
    // reset prg rom bank mode
    self.registers.control |= 0b1100
  }

  fn prg_ram_enabled(&self) -> bool {
    false
  }
}

impl MapperActions for Mmc1 {
  fn map_read(&mut self, address: u16) -> u16 {
    0
  }

  fn clock(&mut self) {
    if self.registers.write_occurred > 0 {
      self.registers.write_occurred -= 1;
    }
  }

  fn map_write(&mut self, address: u16, val: u8) -> Option<u16> {
      match address {
        0x6000..=0x7fff if self.prg_ram_enabled() => {
          Some(self.prg_ram_banks.translate(address))
        }
        0x8000..=0xffff => {
          if self.registers.write_occurred > 0 {
            return None;
          }
          self.registers.write_occurred = 2;

          if (val >> 7) & 0b1 == 1 {
            self.reset_shift_register();
          } else {
            if self.registers.current_shift_write < 5 {
              self.registers.shift |= (val & 0b1) << self.registers.current_shift_write;

              self.registers.current_shift_write += 1;
            } else {
              // write to the register designated in the address
              match address {
                0x8000..=0x9fff => self.registers.control = self.registers.shift,
                0xa000..=0xbfff => self.registers.chr0 = self.registers.shift,
                0xc000..=0xdfff => self.registers.chr0 = self.registers.shift,
                0xe000..=0xffff => self.registers.prg = self.registers.shift,
                _ => panic!("not possible")
              }

              self.registers.shift = 0;
              self.registers.current_shift_write = 0;
              self.update_banks(address);
            }
          }

          None
        },
        _ => None
      }

  }
}

impl Mmc1 {
  fn update_banks(&mut self, address: u16) {
    if (self.registers.control >> 1) & 0b1 == 1 {

    }
  }
}

pub struct MemoryBanks {
  start: usize,
  end: usize,
  size: usize,
  window: usize,
  shift: usize,
  mask: usize,
  banks: Vec<usize>,
  page_count: usize,
}

impl MemoryBanks {
  pub fn new(start: usize, end: usize, capacity: usize, window: usize) -> Self {
    let size = end - start;

    let mut banks: Vec<usize> = vec![0; (size + 1) / window];

    for (i, bank) in banks.iter_mut().enumerate() {
      *bank = i * window;
    }

    let mut page_count = capacity / window;

    if page_count == 0 {
      page_count = 1;
    }

    Self {
      start,
      end,
      size,
      window,
      shift: window.trailing_zeros() as usize,
      mask: page_count - 1,
      banks,
      page_count
    }
  }

  pub fn translate(&self, address: u16) -> u16 {
    0
  }
}