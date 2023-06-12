use crate::nes::CPU;
use crate::nes::cpu::CpuFlags;

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
      0x00 => self.brk(),
      0x06 => self.asl(AddressingMode::ZeroPage),
      0x0a => self.asl_accumulator(),
      0x0e => self.asl(AddressingMode::Absolute),
      // BPL
      0x10 => self.branch(!self.registers.p.contains(CpuFlags::NEGATIVE)),
      0x16 => self.asl(AddressingMode::ZeroPageX),
      0x1e => self.asl(AddressingMode::AbsoluteX),
      // CLC
      0x18 => self.registers.p.remove(CpuFlags::CARRY),
      0x21 => self.and(AddressingMode::IndirectX),
      0x24 => self.bit(AddressingMode::ZeroPage),
      0x25 => self.and(AddressingMode::ZeroPage),
      0x29 => self.and(AddressingMode::Immediate),
      0x2c => self.bit(AddressingMode::Absolute),
      // BMI
      0x30 => self.branch(self.registers.p.contains(CpuFlags::NEGATIVE)),
      0x31 => self.and(AddressingMode::IndirectY),
      0x2d => self.and(AddressingMode::Absolute),
      0x35 => self.and(AddressingMode::ZeroPageX),
      0x39 => self.and(AddressingMode::AbsoluteY),
      0x3d => self.and(AddressingMode::AbsoluteX),
      // BVC
      0x50 => self.branch(!self.registers.p.contains(CpuFlags::OVERFLOW)),
      // CLI
      0x58 => self.registers.p.remove(CpuFlags::INTERRUPT_DISABLE),
      0x61 => self.adc(AddressingMode::IndirectX),
      0x65 => self.adc(AddressingMode::ZeroPage),
      0x69 => self.adc(AddressingMode::Immediate),
      0x6d => self.adc(AddressingMode::Absolute),
      // BVS
      0x70 => self.branch(self.registers.p.contains(CpuFlags::OVERFLOW)),
      0x71 => self.adc(AddressingMode::IndirectY),
      0x75 => self.adc(AddressingMode::ZeroPageX),
      0x7d => self.adc(AddressingMode::AbsoluteX),
      0x79 => self.adc(AddressingMode::AbsoluteY),
      // BCC
      0x90 => self.branch(!self.registers.p.contains(CpuFlags::CARRY)),
      0xa5 =>  self.lda(AddressingMode::ZeroPage),
      0xa1 => self.lda(AddressingMode::IndirectX),
      0xa8 => self.tay(),
      0xa9 => self.lda(AddressingMode::Immediate),
      0xad => self.lda(AddressingMode::Absolute),
      // BCS
      0xb0 => self.branch(self.registers.p.contains(CpuFlags::CARRY)),
      0xb1 => self.lda(AddressingMode::IndirectY),
      0xb5 => self.lda(AddressingMode::ZeroPageX),
      // CLV
      0xb8 => self.registers.p.remove(CpuFlags::OVERFLOW),
      0xb9 => self.lda(AddressingMode::AbsoluteY),
      0xbd => self.lda(AddressingMode::AbsoluteX),
      0xaa => self.tax(),
      0xe8 => self.inx(),
      // BNE
      0xd0 => self.branch(!self.registers.p.contains(CpuFlags::ZERO)),
      // CLD
      0xd8 => self.registers.p.remove(CpuFlags::DECIMAL_MODE),
      // NOP
      0xea => return,
      // BEQ
      0xf0 => self.branch(self.registers.p.contains(CpuFlags::ZERO)),
      _ => println!("unknown instruction received: {}", format!("{:X}", op_code))
    }
  }

  fn inx(&mut self) {
    self.registers.x += 1;

    self.set_zero_and_negative_flags(self.registers.x);
  }

  fn bit(&mut self, mode: AddressingMode) {
    let address = self.get_operand_address(mode);

    let val = self.mem_read(address);

    let result = val & self.registers.a;

    self.registers.p.set(CpuFlags::ZERO, result == 0);
    self.registers.p.set(CpuFlags::OVERFLOW, (val >> 6) & 0b1 == 1);
    self.registers.p.set(CpuFlags::NEGATIVE, val >> 7 == 1);
  }

  fn asl_accumulator(&mut self) {
    if self.registers.a >> 7 == 1 {
      self.registers.p.insert(CpuFlags::CARRY);
    } else {
      self.registers.p.remove(CpuFlags::CARRY);
    }

    self.registers.a = self.registers.a << 1;

    self.set_zero_and_negative_flags(self.registers.a);
  }

  fn asl(&mut self, mode: AddressingMode) {
    let address = self.get_operand_address(mode);

    let mut val = self.mem_read(address);

    if val >> 7 == 1 {
      self.registers.p.insert(CpuFlags::CARRY);
    } else {
      self.registers.p.remove(CpuFlags::CARRY);
    }

    val = val << 1;

    self.set_zero_and_negative_flags(val);

    self.mem_write(address, val);
  }

  fn brk(&mut self) {
    self.push_to_stack_u16(self.registers.pc);
    self.push_to_stack(self.registers.p.bits());

    let address = self.mem_read_u16(0xfffe);

    self.registers.p.insert(CpuFlags::BREAK);

    self.registers.pc = address;
  }

  fn and(&mut self, mode: AddressingMode) {
    let address = self.get_operand_address(mode);

    let val = self.mem_read(address);

    self.registers.a &= val;

    self.set_zero_and_negative_flags(self.registers.a);
  }

  fn get_operand_address(&mut self, mode: AddressingMode) -> u16 {
    match mode {
      AddressingMode::Immediate => {
        let address = self.registers.pc;

        self.registers.pc += 1;

        address
      }
      AddressingMode::ZeroPage => {
        let address = self.memory[self.registers.pc as usize] as u16;

        self.registers.pc += 1;

        address
      }
      AddressingMode::ZeroPageX => {
        let base_address = self.memory[self.registers.pc as usize];

        let address = base_address.wrapping_add(self.registers.x) as u16;

        self.registers.pc += 1;

        address
      }
      AddressingMode::ZeroPageY => {
        let base_address = self.mem_read(self.registers.pc);

        let address = base_address.wrapping_add(self.registers.y) as u16;

        self.registers.pc += 1;

        address
      }
      AddressingMode::Absolute => {
        let address = self.mem_read_u16(self.registers.pc);

        self.registers.pc += 2;

        address
      }
      AddressingMode::AbsoluteX => {
        let base_address = self.mem_read_u16(self.registers.pc);

        let address = base_address.wrapping_add(self.registers.x as u16);

        self.registers.pc += 2;

        address
      }
      AddressingMode::AbsoluteY => {
        let base_address = self.mem_read_u16(self.registers.pc);

        let address = base_address.wrapping_add(self.registers.y as u16);

        self.registers.pc += 2;

        address
      }
      AddressingMode::IndirectX => {
        let address = self.indirect_address(self.registers.x);

        self.registers.pc += 1;

        address
      }
      AddressingMode::IndirectY => {
         let address = self.indirect_address(self.registers.y);

         self.registers.pc += 1;
      }
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

  fn absolute_offset_address() {

  }

  fn lda(&mut self, mode: AddressingMode) {
    let address = self.get_operand_address(mode);

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

    self.set_zero_and_negative_flags(self.registers.y);
  }

  fn set_zero_and_negative_flags(&mut self, result: u8) {
    if result == 0 {
      self.registers.p.insert(CpuFlags::ZERO);
    } else {
      self.registers.p.remove(CpuFlags::ZERO);
    }

    if result & (0b1 << 7) == 1 {
      self.registers.p.insert(CpuFlags::NEGATIVE);
    } else {
      self.registers.p.remove(CpuFlags::NEGATIVE);
    }
  }

  fn adc(&mut self, mode: AddressingMode) {
    let address = self.get_operand_address(mode);

    let val = self.mem_read(address);

    let carry = if self.registers.p.contains(CpuFlags::CARRY) { 1 } else { 0 };

    let result = self.registers.a.wrapping_add(val + carry);

    if self.registers.a > result {
      self.registers.p.insert(CpuFlags::CARRY);
    } else {
      self.registers.p.remove(CpuFlags::CARRY);
    }

    if (val ^ result) & (result ^ self.registers.a) & 0b10000000 != 0 {
      self.registers.p.insert(CpuFlags::OVERFLOW);
    } else {
      self.registers.p.remove(CpuFlags::OVERFLOW);
    }

    self.registers.a = result;
  }

  fn branch(&mut self, condition: bool) {
    if condition {
      let val = self.mem_read(self.registers.pc) as i8;

      self.registers.pc += 1;

      self.registers.pc = self.registers.pc.wrapping_add_signed(val as i16);
    } else {
      self.registers.pc += 1;
    }
  }

  fn todo(&mut self) {

  }
}