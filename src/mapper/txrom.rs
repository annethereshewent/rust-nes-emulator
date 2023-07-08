use crate::cartridge::{Cartridge, Mirroring};

use super::{MapperActions, BankType};

const PRG_ROM_BANK_SIZE: usize = 8192;
const CHR_BANK_SIZE: usize = 1024;

const CHR_RAM_SIZE: usize = 8192;
const PRG_RAM_SIZE: usize = 8192;

pub struct Txrom {
  prg_rom_banks: [usize; 4],
  chr_banks: [usize; 8],
  mirroring: Mirroring,
  registers: TxromRegisters,
  prg_page_size: u8,
  chr_page_size: u8,
  irq_pending: bool
}

struct TxromRegisters {
  bank_select: u8,
  bank_data: u8,
  irq_latch: u8,
  irq_counter: u8,
  irq_reload: bool,
  irq_enable: bool,
  last_clock: u8
}

impl Txrom {
  pub fn load(cartridge: &mut Cartridge) -> Self {
    let chr_len = if cartridge.chr_rom.is_empty() {
      cartridge.chr_ram.resize(CHR_RAM_SIZE, 0);

      CHR_RAM_SIZE
    } else {
      cartridge.chr_rom.len()
    };

    cartridge.prg_ram.resize(PRG_RAM_SIZE, 0);

    let mut txrom = Self {
      prg_rom_banks: [0; 4],
      chr_banks: [0; 8],
      mirroring: cartridge.mirroring,
      prg_page_size: (cartridge.prg_rom.len() / PRG_ROM_BANK_SIZE) as u8,
      chr_page_size: (chr_len / CHR_BANK_SIZE) as u8,
      irq_pending: false,
      registers: TxromRegisters {
        bank_select: 0,
        bank_data: 0,
        irq_latch: 0,
        irq_reload: false,
        irq_enable: false,
        irq_counter: 0,
        last_clock: 0
      }
    };

    txrom.prg_rom_banks[2] = txrom.get_bank_address(txrom.prg_page_size-2, txrom.prg_page_size, PRG_ROM_BANK_SIZE);
    txrom.prg_rom_banks[3] = txrom.get_bank_address(txrom.prg_page_size -1, txrom.prg_page_size, PRG_ROM_BANK_SIZE);

    txrom
  }

  fn update_chr_banks(&mut self, mode: u8, bank_select: u8) {
    let bank = self.registers.bank_data;
    if mode == 1 {
      match bank_select {
        0 => {
          // swap bank at 1000-17ff (2kb bank, bank indexes 4-5)
          self.chr_banks[4] = self.get_bank_address(bank, self.chr_page_size, CHR_BANK_SIZE);
          self.chr_banks[5] = self.get_bank_address(bank + 1, self.chr_page_size, CHR_BANK_SIZE);
        }
        1 => {
          // 1800-1fff (2kb bank, bank indexes 6-7)
          self.chr_banks[6] = self.get_bank_address(bank, self.chr_page_size, CHR_BANK_SIZE);
          self.chr_banks[7] = self.get_bank_address(bank + 1, self.chr_page_size, CHR_BANK_SIZE);
        }
        2 => {
          // 0000-03ff (1kb bank, bank index 0)
          self.chr_banks[0] = self.get_bank_address(bank, self.chr_page_size, CHR_BANK_SIZE);
        }
        3 => {
          // 0400-07ff (1kb bank, bank index 1)
          self.chr_banks[1] = self.get_bank_address(bank, self.chr_page_size, CHR_BANK_SIZE);
        }
        4 => {
          // 0800-0bff (1kb bank, bank index 2)
          self.chr_banks[2] = self.get_bank_address(bank, self.chr_page_size, CHR_BANK_SIZE);
        }
        5 => {
          // 0c00-0fff (1kb bank, bank index 3)
          self.chr_banks[3] = self.get_bank_address(bank, self.chr_page_size, CHR_BANK_SIZE);
        }
        _ => panic!("not possible")
      }
    } else {
      match bank_select {
        0 => {
          // 0000 - 07ff (2kb bank, bank indexes 0-1)
          self.chr_banks[0] = self.get_bank_address(bank, self.chr_page_size, CHR_BANK_SIZE);
          self.chr_banks[1] = self.get_bank_address(bank + 1, self.chr_page_size, CHR_BANK_SIZE);
        }
        1 => {
          // 0800 - 0fff (2kb bank, bank indexes 2-3)
          self.chr_banks[2] = self.get_bank_address(bank, self.chr_page_size, CHR_BANK_SIZE);
          self.chr_banks[3] = self.get_bank_address(bank + 1, self.chr_page_size, CHR_BANK_SIZE);
        }
        2 => {
          // 1000 - 13ff (1kb bank, bank index 4)
          self.chr_banks[4] = self.get_bank_address(bank, self.chr_page_size, CHR_BANK_SIZE);
        }
        3 => {
          // 1400 - 17ff (1kb bank, bank index 5)
          self.chr_banks[5] = self.get_bank_address(bank, self.chr_page_size, CHR_BANK_SIZE);
        }
        4 => {
          // 1800 - 1bff (1kb bank, bank index 6)
          self.chr_banks[6] = self.get_bank_address(bank, self.chr_page_size, CHR_BANK_SIZE);
        }
        5 => {
          // 1c00 - 1fff (1kb bank, bank index 7)
          self.chr_banks[7] = self.get_bank_address(bank, self.chr_page_size, CHR_BANK_SIZE);
        }
        _ => panic!("not possible")
      }
    }
  }

  fn get_bank_address(&self, bank: u8, page_size: u8, bank_size: usize) -> usize {
    (bank % page_size) as usize * bank_size
  }

  fn update_prg_banks(&mut self, mode: u8, bank_select: u8) {
    let bank = self.registers.bank_data;

    if bank_select == 7 {
      // swap bank at a000-bfff (bank 2)
      self.prg_rom_banks[1] = self.get_bank_address(bank, self.prg_page_size, PRG_ROM_BANK_SIZE);
    } else {
      if mode == 0 {
        // bank at 0x8000-9fff (bank 1) swappable,
        self.prg_rom_banks[0] = self.get_bank_address(bank, self.prg_page_size, PRG_ROM_BANK_SIZE);
        // bank at c000-dfff (bank 3) fixed to second to last bank
        self.prg_rom_banks[2] = self.get_bank_address(self.prg_page_size - 2, self.prg_page_size, PRG_ROM_BANK_SIZE);
      } else {
        // c000-dfff (bank 3) swappable,
        self.prg_rom_banks[2] = self.get_bank_address(bank, self.prg_page_size, PRG_ROM_BANK_SIZE);
        // 8000-9fff (bank 1) fixed to second to last bank
        self.prg_rom_banks[0] = self.get_bank_address(self.prg_page_size - 2, self.prg_page_size, PRG_ROM_BANK_SIZE);
      }
    }
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

  fn get_chr_bank_number(&self, address: u16) -> usize {
    (address as usize) / CHR_BANK_SIZE
  }

  fn get_prg_bank_number(&self, address: u16) -> usize {
    (address - 0x8000) as usize / PRG_ROM_BANK_SIZE
  }

  fn update_banks(&mut self) {
    let prg_bank_mode = (self.registers.bank_select >> 6) & 0b1;
    let chr_mode = self.registers.bank_select >> 7;
    let bank_select = self.registers.bank_select & 0b111;

    match bank_select {
      0 | 1 | 2 | 3 | 4 | 5 => self.update_chr_banks(chr_mode, bank_select),
      6 | 7 => self.update_prg_banks(prg_bank_mode, bank_select),
      _ => panic!("impossible")
    }
  }

  fn clock_irq(&mut self, address: u16) {
    if address < 0x2000 {
      let next_clock = ((address >> 12) & 0b1) as u8;
      let last: u8 = 0;
      let next: u8 = 1;

      if self.registers.last_clock == last && next_clock == next {
        let counter = self.registers.irq_counter;
        if self.registers.irq_counter == 0 || self.registers.irq_reload {
          self.registers.irq_counter = self.registers.irq_latch;
        } else {
          self.registers.irq_counter -= 1;
        }
        if ((counter & 0b1) == 1 || self.registers.irq_reload) && self.registers.irq_enable && self.registers.irq_counter == 0 {
          self.irq_pending = true
        }
        self.registers.irq_reload = false;
      }
      self.registers.last_clock = next_clock;
    }
  }
}

impl MapperActions for Txrom {

  fn irq_pending(&self) -> bool {
    self.irq_pending
  }

  fn set_irq_pending(&mut self, val: bool) {
      self.irq_pending = val;
  }

  fn ppu_bus_write(&mut self, address: u16, _val: u8) {
    self.clock_irq(address);
  }

  fn mirroring(&self) -> Mirroring {
    self.mirroring
  }

  fn mem_read(&mut self, address: u16) -> Option<usize> {
    match address {
      0x0000..=0x1fff => self.translate_address(address, BankType::Chr),
      0x6000..=0x7fff => Some((address - 0x6000) as usize),
      0x8000..=0xffff => self.translate_address(address, BankType::Prg),
      _ => panic!("not possible")
    }
  }
  fn mem_write(&mut self, address: u16, val: u8) -> Option<usize> {
    match address {
      0x0000..=0x1fff => self.translate_address(address, BankType::Chr),
      0x6000..=0x7fff => Some((address - 0x6000) as usize),
      0x8000..=0x9fff => {
        if address %2 == 0 {
          self.registers.bank_select = val;
        } else {
          self.registers.bank_data = val;
          self.update_banks();
        }
        None
      }
      0xa000..=0xbfff => {
        if address %2 == 0 {
          self.mirroring = if (val & 0b1) == 0 {
            Mirroring::Vertical
          } else {
            Mirroring::Horizontal
          };
        } else {
          // per https://www.nesdev.org/wiki/MMC3,
          // "Many emulators choose not to implement them as part of iNES Mapper 4 to avoid an incompatibility with the MMC6."
        }
        None
      }
      0xc000..=0xdfff => {
        if address %2 == 0 {
          self.registers.irq_latch = val
        } else {
          self.registers.irq_reload = true;

        }
        None
      }
      0xe000..=0xffff => {
        if address %2 == 0 {
          self.registers.irq_enable = false;
          self.irq_pending = false;
        } else {
          self.registers.irq_enable = true;
        }
        None
      }
      _ => None
    }
  }
}