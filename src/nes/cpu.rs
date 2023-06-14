pub mod op_codes;

use bitflags::bitflags;

use super::cartridge::Cartridge;

pub struct CPU {
  pub registers: Registers,
  pub memory: [u8; 0x10000]
}

const STACK_BASE_ADDR: u16 = 0x0100;
const STACK_START: u8 = 0xfd;

pub struct Registers {
  pub a: u8,
  pub pc: u16,
  pub x: u8,
  pub y: u8,
  pub sp: u8,
  pub p: CpuFlags
}

bitflags! {
  pub struct CpuFlags: u8 {
    const CARRY             = 0b1;
    const ZERO              = 0b10;
    const INTERRUPT_DISABLE = 0b100;
    const DECIMAL_MODE      = 0b1000;
    const BREAK             = 0b10000;
    const BREAK2            = 0b100000;
    const OVERFLOW          = 0b1000000;
    const NEGATIVE          = 0b10000000;
  }
}

impl CpuFlags {
  // there doesn't seem to be a way to just se the value to the byte easily so it has to be done like this
  pub fn set_bits(&mut self, byte: u8) {
    self.set(Self::CARRY, byte & 0b1 == 1);
    self.set(Self::ZERO, (byte >> 1) & 0b1 == 1);
    self.set(Self::INTERRUPT_DISABLE, (byte >> 2) & 0b1 == 1);
    self.set(Self::DECIMAL_MODE, (byte >> 3) & 0b1 == 1);
    self.set(Self::BREAK, (byte >> 4) & 0b1 == 1);
    self.set(Self::BREAK2, (byte >> 5) & 0b1 == 1);
    self.set(Self::OVERFLOW, (byte >> 6) & 0b1 == 1);
    self.set(Self::NEGATIVE, (byte >> 7) & 0b1 == 1);
  }
}

impl CPU {
  pub fn new(cartridge: Cartridge) -> Self {
    let mut cpu = CPU {
      registers: Registers {
        a: 0,
        pc: 0,
        p: CpuFlags::from_bits_truncate(0b100100),
        x: 0,
        y: 0,
        sp: STACK_START
      },
      memory: [0; 0x10000]
    };

    cpu.load_game(cartridge.prg_rom);

    cpu
  }

  pub fn mem_read(&self, address: u16) -> u8 {
    match address {
      0x0000 ..= 0x1fff => self.memory[(address & 0b11111111111) as usize],
      0x2000 ..= 0x3fff => self.memory[(address & 0b100000_00000111) as usize],
      _ => self.memory[address as usize]
    }
  }

  pub fn mem_write(&mut self, address: u16, value: u8) {
    match address {
      0x0000 ..= 0x1fff => self.memory[(address & 0b11111111111) as usize] = value,
      0x2000 ..= 0x3fff => self.memory[(address & 0b100000_00000111) as usize] = value,
      0x8000 ..= 0xffff => panic!("attempting to write to rom"),
      _ => println!("ignoring write to address {address}")
    };
  }

  pub fn mem_write_u16(&mut self, address: u16, value: u16) {
    let lower_byte = (value & 0b11111111) as u8;
    let upper_byte = ((value >> 8) & 0b11111111) as u8;

    self.mem_write(address, lower_byte);
    self.mem_write(address + 1, upper_byte);
  }

  pub fn mem_read_u16(&self, address: u16) -> u16 {
    let low_byte = self.mem_read(address) as u16;
    let high_byte = self.mem_read(address + 1) as u16;

    (high_byte << 8) | low_byte
  }

  pub fn load_game(&mut self, rom: Vec<u8>) {
    self.memory[0x8000 .. (0x8000 + rom.len())].copy_from_slice(&rom[..]);
    self.registers.pc = 0x8000;
  }

  pub fn tick(&mut self) {
    let op_code = self.memory[self.registers.pc as usize];

    self.registers.pc += 1;

    self.decode(op_code);
  }

  pub fn push_to_stack(&mut self, val: u8) {
    self.mem_write(STACK_BASE_ADDR + self.registers.sp as u16, val);

    self.registers.sp = self.registers.sp.wrapping_sub(1);
  }

  pub fn push_to_stack_u16(&mut self, val: u16) {
    self.mem_write_u16(STACK_BASE_ADDR + self.registers.sp as u16, val);

    self.registers.sp = self.registers.sp.wrapping_sub(2);
  }

  pub fn pop_from_stack(&mut self) -> u8 {
    self.registers.sp = self.registers.sp.wrapping_add(1);

    self.mem_read(STACK_BASE_ADDR + self.registers.sp as u16)
  }

  pub fn pop_from_stack_u16(&mut self) -> u16 {
    let lower_byte = self.pop_from_stack() as u16;
    let upper_byte = self.pop_from_stack() as u16;

    upper_byte << 8 | lower_byte
  }

  pub fn cycle(&mut self, _cycles: u8) {
    // TODO
  }
}