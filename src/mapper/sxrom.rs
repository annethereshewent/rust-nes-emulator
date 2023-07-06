use crate::cartridge::{Cartridge, Mirroring};

use super::MapperActions;

const PRG_RAM_SIZE: usize = 8192;
const CHR_RAM_SIZE: usize = 8192;

const CHR_BANK_SIZE: usize = 4096;
const _PRG_RAM_BANK_SIZE: usize = 8192;
const PRG_ROM_BANK_SIZE: usize = 16_384;

enum BankType {
  Chr,
  Prg
}

struct SxromRegisters {
  write_occurred: i8,
  shift: u8,
  control: u8,
  chr0: u8,
  chr1: u8,
  prg: u8,
  current_shift_write: u8
}
pub struct Sxrom {
  registers: SxromRegisters,
  mirroring: Mirroring,
  chr_select: bool,
  prg_rom_banks: [usize; 2],
  chr_banks: [usize; 2],
  prg_rom_page_size: u8,
  chr_page_size: u8,
  chr_shift: u32,
  prg_shift: u32
}

impl Sxrom {
  pub fn load(cartridge: &mut Cartridge) -> Self {
    cartridge.chr_ram.resize(CHR_RAM_SIZE, 0);
    cartridge.prg_ram.resize(PRG_RAM_SIZE, 0);

    let chr_banks: [usize; 2] = [0, CHR_BANK_SIZE];
    let prg_rom_banks: [usize; 2] = [0, PRG_ROM_BANK_SIZE];

    let chr_length = if cartridge.chr_rom.is_empty() { cartridge.chr_ram.len() } else { cartridge.chr_rom.len() };

    let mut prg_rom_page_size: u8 = (cartridge.prg_rom.len() / PRG_ROM_BANK_SIZE) as u8;
    let mut chr_page_size: u8 = (chr_length / CHR_BANK_SIZE) as u8;

    if prg_rom_page_size == 0 {
      prg_rom_page_size = 1;
    }
    if chr_page_size == 0 {
      chr_page_size = 1;
    }

    let mut sxrom = Self {
      registers: SxromRegisters {
        write_occurred: 0,
        shift: 0x0,
        control: 0xc,
        chr0: 0,
        chr1: 0,
        prg: 0,
        current_shift_write: 0
      },
      mirroring: Mirroring::SingleScreenA,
      chr_select: cartridge.prg_rom.len() == 0x80000,
      prg_rom_banks,
      chr_banks,
      prg_rom_page_size,
      chr_page_size,
      chr_shift: CHR_BANK_SIZE.trailing_zeros(),
      prg_shift: PRG_ROM_BANK_SIZE.trailing_zeros()
    };


    sxrom.update_banks(0);

    sxrom
  }

  fn reset_shift_register(&mut self) {
    self.registers.shift = 0;
    self.registers.current_shift_write = 0;
    // reset prg rom bank mode
    self.registers.control |= 0b1100
  }

  fn prg_ram_enabled(&self) -> bool {
    (self.registers.prg >> 4) & 0b1 == 0
  }

  fn update_banks(&mut self, address: u16) {
    let mirroring_type = self.registers.control & 0b11;
    self.mirroring = match mirroring_type {
      0 => Mirroring::SingleScreenA,
      1 => Mirroring::SingleScreenB,
      2 => Mirroring::Vertical,
      3 => Mirroring::Horizontal,
      _ => panic!("impossible")
    };

    let chr_bank_mode = (self.registers.control >> 4) & 0b1;

    if chr_bank_mode == 1 {
      self.chr_banks[0] = self.get_bank_address(self.registers.chr0, self.chr_page_size-1, self.chr_shift);
      self.chr_banks[1] = self.get_bank_address(self.registers.chr1, self.chr_page_size-1, self.chr_shift);
    } else {
      let bank = self.registers.chr0 & 0b11110; // ignore lower bit of register in mode 0
      let new_address = self.get_bank_address(bank, self.chr_page_size-1, self.chr_shift);

      self.chr_banks[0] = new_address;
      self.chr_banks[1] = new_address + CHR_BANK_SIZE;
    }

    // finally set prg banks
    let prg_mode = self.registers.control >> 2 & 0b11;

    let extra_register = if matches!(address, 0xc000..=0xdfff) && chr_bank_mode == 1 {
      self.registers.chr1
    } else {
      self.registers.chr0
    };

    let bank_select = if self.chr_select {
      extra_register & 0b10000
    } else {
      0
    };

    let prg_bank = self.registers.prg & 0b1111; // only first 4 bits matter, last bit is for prg ram enable

    // per https://www.nesdev.org/wiki/MMC1#PRG_bank_(internal,_$E000-$FFFF)
    match prg_mode {
      // switch 32 KB at $8000, ignoring low bit of bank number;
      0 | 1 => {
        let bank_number = bank_select | (prg_bank & 0b11110);

        let new_address = self.get_bank_address(bank_number, self.prg_rom_page_size-1, self.prg_shift);

        self.prg_rom_banks[0] = new_address;
        self.prg_rom_banks[1] = new_address + PRG_ROM_BANK_SIZE;
      }
      // fix first bank at $8000 and switch 16 KB bank at $C000;
      2 => {
        self.prg_rom_banks[0] = self.get_bank_address(bank_select, self.prg_rom_page_size-1, self.prg_shift);
        self.prg_rom_banks[1] = self.get_bank_address(bank_select | prg_bank, self.prg_rom_page_size-1, self.prg_shift);
      }
      // fix last bank at $C000 and switch 16 KB bank at $8000
      3 => {
        self.prg_rom_banks[0] = self.get_bank_address(bank_select | prg_bank, self.prg_rom_page_size-1, self.prg_shift);
        self.prg_rom_banks[1] = self.get_bank_address((self.prg_rom_page_size - 1) | bank_select, self.prg_rom_page_size-1, self.prg_shift);
      },
      _ => panic!("can't happen")
    }
  }

  fn get_bank_address(&self, bank: u8, mask: u8, shift: u32) -> usize {
    ((bank & mask) as usize) << shift
  }

  fn translate_address(&self, address: u16, bank_type: BankType) -> Option<usize> {
    match bank_type {
      BankType::Chr => {
        let page = self.chr_banks[self.get_chr_bank_number(address)];
        Some(page | (address as usize) & (CHR_BANK_SIZE - 1))
      }
      BankType::Prg => {
        let page = self.prg_rom_banks[self.get_prg_bank_number(address)];

        Some(page | (address as usize) & (PRG_ROM_BANK_SIZE - 1))
      }
    }
  }

  fn get_prg_bank_number(&self, address: u16) -> usize {
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

  fn get_chr_bank_number(&self, address: u16) -> usize {
    match address {
      0x0000..=0x0fff => {
        0
      }
      0x1000..=0x1fff => {
        1
      }
      _ => panic!("not possible")
    }
  }
}

impl MapperActions for Sxrom {
  fn mem_read(&mut self, address: u16) -> Option<usize> {
    match address {
      0x0000..=0x1fff => self.translate_address(address, BankType::Chr),
      0x6000..=0x7fff if self.prg_ram_enabled() => Some(address as usize),
      0x8000..=0xffff => self.translate_address(address, BankType::Prg),
      _ => panic!("not possible")
    }
  }

  fn tick(&mut self, cycles: u8) {
    if self.registers.write_occurred > 0 {
      self.registers.write_occurred -= cycles as i8;
    }
  }

  fn mem_write(&mut self, address: u16, val: u8) -> Option<usize> {
      match address {
        0x0000..=0x1fff => self.translate_address(address, BankType::Chr),
        0x6000..=0x7fff if self.prg_ram_enabled() => Some(address as usize),
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

              if self.registers.current_shift_write == 5 {
                // write to the register designated in the address
                match address {
                  0x8000..=0x9fff => self.registers.control = self.registers.shift,
                  0xa000..=0xbfff => self.registers.chr0 = self.registers.shift,
                  0xc000..=0xdfff => self.registers.chr1 = self.registers.shift,
                  0xe000..=0xffff => self.registers.prg = self.registers.shift,
                  _ => panic!("not possible")
                }

                self.registers.shift = 0;
                self.registers.current_shift_write = 0;
                self.update_banks(address);
              }
            }
          }
          None
        },
        _ => None
      }

  }
}
