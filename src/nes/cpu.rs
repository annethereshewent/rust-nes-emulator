pub mod op_codes;

pub struct CPU {
  pub registers: Registers,
  pub program: Vec<u8>
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
      program: rom
    }
  }

  pub fn tick(&mut self) {
    let op_code = if self.program.len() > 0 {
      self.program[self.registers.pc as usize]
    } else {
      0
    };

    self.decode(op_code);

    self.registers.pc += 1;
  }
}