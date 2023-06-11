use crate::nes::CPU;

enum AddressingMode {
  Immediate,
  ZeroPage,
  ZeroPageX,
  ZeroPageY,
  Absolute,
  AbsoluteX,
  AbsoluteY,
  IndirectX,
  IndirectY,
  NoneAddressing,

}

impl CPU {
  pub fn decode(&mut self, op_code: u8) {
    match op_code {
      0x00 => return,
      0xa5 =>  {
        self.lda(AddressingMode::ZeroPage);
        self.registers.pc += 1;
      }
      0xa1 => {
        self.lda(AddressingMode::IndirectX);
        self.registers.pc += 1;
      }
      0xa8 => self.tay(),
      0xa9 => {
        self.lda(AddressingMode::Immediate);
        self.registers.pc += 1;
      }
      0xad => {
        self.lda(AddressingMode::Absolute);
        self.registers.pc += 2;
      }
      0xb1 => {
        self.lda(AddressingMode::IndirectY);
        self.registers.pc += 1;
      }
      0xb5 => {
        self.lda(AddressingMode::ZeroPageX);
        self.registers.pc += 1;
      }
      0xb9 => {
        self.lda(AddressingMode::AbsoluteY);
        self.registers.pc += 2;
      }
      0xbd => {
        self.lda(AddressingMode::AbsoluteX);
        self.registers.pc += 2;
      }

      0xaa => self.tax(),
      0xe8 => self.inx(),
      _ => self.todo()
    }
  }

  fn inx(&mut self) {
    self.registers.x += 1;

    self.set_zero_and_negative_flags(self.registers.x);
  }

  fn get_operand_address(&self, mode: AddressingMode) -> u16 {
    match mode {
      AddressingMode::Immediate => self.registers.pc,
      AddressingMode::ZeroPage => self.memory[self.registers.pc as usize] as u16,
      AddressingMode::ZeroPageX => {
        let base_address = self.memory[self.registers.pc as usize];

        base_address.wrapping_add(self.registers.x) as u16
      }
      AddressingMode::ZeroPageY => {
        let base_address = self.mem_read(self.registers.pc);

       base_address.wrapping_add(self.registers.y) as u16
      }
      AddressingMode::Absolute => self.mem_read_u16(self.registers.pc),
      AddressingMode::AbsoluteX => {
        let base_address = self.mem_read_u16(self.registers.pc);

        base_address.wrapping_add(self.registers.x as u16)
      }
      AddressingMode::AbsoluteY => {
        let base_address = self.mem_read_u16(self.registers.pc);

        base_address.wrapping_add(self.registers.y as u16)
      }
      AddressingMode::IndirectX => self.indirect_address(self.registers.x),
      AddressingMode::IndirectY => self.indirect_address(self.registers.y),
      AddressingMode::NoneAddressing => panic!("mode is not supported")
    }
  }

  fn indirect_address(&self, offset: u8) -> u16 {
    let base_address = self.mem_read(self.registers.pc);

    let actual_address = base_address.wrapping_add(offset);

    let low_byte = self.mem_read(actual_address as u16) as u16;
    let high_byte = self.mem_read(actual_address.wrapping_add(1) as u16) as u16;

    (high_byte << 8) | low_byte
  }

  fn lda(&mut self, addressing_mode: AddressingMode) {
    let address = self.get_operand_address(addressing_mode);

    let val = self.mem_read(address);

    self.registers.a = val;

    self.set_zero_and_negative_flags(val);
  }

  fn tax(&mut self) {
    self.registers.x = self.registers.a;

    self.set_zero_and_negative_flags(self.registers.x);
  }

  fn tay(&mut self) {
    self.registers.y = self.registers.a;

    self.set_zero_and_negative_flags((self.registers.y);)
  }

  fn set_zero_and_negative_flags(&mut self, result: u8) {
    if result == 0 {
      self.registers.p = self.registers.p | (0b1 << 1);
    } else {
      self.registers.p = self.registers.p & !(0b1 << 1);
    }

    if result & (0b1 << 7) == 1 {
      self.registers.p = self.registers.p | (0b1 << 7);
    } else {
      self.registers.p = self.registers.p & !(0b1 << 7);
    }
  }

  fn todo(&mut self) {

  }
}