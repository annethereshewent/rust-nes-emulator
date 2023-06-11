pub mod op_codes;

pub struct CPU {
  pub registers: Registers,
  pub memory: [u8; 0xffff]
}

pub struct Registers {
  pub a: u8,
  pub pc: u16,
  pub x: u8,
  pub y: u8,
  pub p: u8
}

impl CPU {
  pub fn new(rom: Vec<u8>) -> Self {
    if rom.len() == 0 {
      panic!("invalid file specified")
    }

    CPU {
      registers: Registers {
        a: 0,
        pc: 0,
        p: 0,
        x: 0,
        y: 0
      },
      memory: [0; 0xffff]
    }
  }

  pub fn mem_read(&self, address: u16) -> u8 {
    self.memory[address as usize]
  }

  pub fn mem_read_u16(&self, address: u16) -> u16 {
    let low_byte = self.mem_read(address) as u16;
    let high_byte = self.mem_read(address + 1) as u16;

    (high_byte << 8) | low_byte
  }

  pub fn load_game(&mut self, rom: Vec<u8>) {
    self.memory[0x8000 .. (0x8000 + rom.len())].copy_from_slice(&rom);
    self.registers.pc = 0x8000;
  }

  pub fn tick(&mut self) {
    let op_code = self.memory[self.registers.pc as usize];

    self.registers.pc += 1;

    self.decode(op_code);
  }

  pub fn cycle(&mut self, _cycles: u16) {
    // TODO
  }
}