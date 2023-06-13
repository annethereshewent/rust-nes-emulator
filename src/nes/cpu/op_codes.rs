use std::fmt;

use crate::nes::CPU;
use crate::nes::cpu::CpuFlags;

pub enum AddressingMode {
  Immediate,
  ZeroPage,
  ZeroPageX,
  ZeroPageY,
  Absolute,
  AbsoluteX,
  AbsoluteY,
  Indirect,
  IndirectX,
  IndirectY,
  NoneAddressing,
  Accumulator
}

impl fmt::Display for AddressingMode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      AddressingMode::Immediate => write!(f, "immediate"),
      AddressingMode::ZeroPage => write!(f, "zero page"),
      AddressingMode::ZeroPageX => write!(f, "zero page x"),
      AddressingMode::ZeroPageY => write!(f, "zero page y"),
      AddressingMode::Absolute => write!(f, "absolute"),
      AddressingMode::AbsoluteX => write!(f, "absolute x"),
      AddressingMode::AbsoluteY => write!(f, "absolute y"),
      AddressingMode::Indirect => write!(f, "indirect"),
      AddressingMode::IndirectX => write!(f, "indirect x"),
      AddressingMode::IndirectY => write!(f, "indirect y"),
      AddressingMode::NoneAddressing => write!(f, "none"),
      AddressingMode::Accumulator => write!(f, "accumulator")
    }
  }
}

pub struct Instruction {
  mode: AddressingMode,
  name: &'static str,
  cycles: u8
}

impl Instruction {
  pub fn new(mode: AddressingMode, name: &'static str, cycles: u8) -> Self {
    Instruction {
      mode,
      name,
      cycles
    }
  }
}
lazy_static! {
  pub static ref INSTRUCTIONS: Vec<Instruction> = vec![
    Instruction::new(AddressingMode::Immediate, "BRK", 7), Instruction::new(AddressingMode::IndirectX, "ORA", 6), Instruction::new(AddressingMode::NoneAddressing, "XXX", 2), Instruction::new(AddressingMode::IndirectX, "SLO", 8), Instruction::new(AddressingMode::ZeroPage, "NOP", 3), Instruction::new(AddressingMode::ZeroPage, "ORA", 3), Instruction::new(AddressingMode::ZeroPage, "ASL", 5), Instruction::new(AddressingMode::ZeroPage, "SLO", 5), Instruction::new(AddressingMode::NoneAddressing, "PHP", 3), Instruction::new(AddressingMode::Immediate, "ORA", 2), Instruction::new(AddressingMode::Accumulator, "ASL", 2), Instruction::new(AddressingMode::Immediate, "ANC", 2), Instruction::new(AddressingMode::Absolute, "NOP", 4), Instruction::new(AddressingMode::Absolute, "ORA", 4), Instruction::new(AddressingMode::Absolute, "ASL", 6), Instruction::new(AddressingMode::Absolute, "SLO", 6),
    Instruction::new(AddressingMode::NoneAddressing, "BPL", 2), Instruction::new(AddressingMode::IndirectY, "ORA", 5), Instruction::new(AddressingMode::NoneAddressing, "XXX", 2), Instruction::new(AddressingMode::IndirectY, "SLO", 8), Instruction::new(AddressingMode::ZeroPageX, "NOP", 4), Instruction::new(AddressingMode::ZeroPageX, "ORA", 4), Instruction::new(AddressingMode::ZeroPageX, "ASL", 6), Instruction::new(AddressingMode::ZeroPageX, "SLO", 6), Instruction::new(AddressingMode::NoneAddressing, "CLC", 2), Instruction::new(AddressingMode::AbsoluteY, "ORA", 4), Instruction::new(AddressingMode::NoneAddressing, "NOP", 2), Instruction::new(AddressingMode::AbsoluteY, "SLO", 7), Instruction::new(AddressingMode::AbsoluteX, "IGN", 4), Instruction::new(AddressingMode::AbsoluteX, "ORA", 4), Instruction::new(AddressingMode::AbsoluteX, "ASL", 7), Instruction::new(AddressingMode::AbsoluteX, "SLO", 7),
    Instruction::new(AddressingMode::Absolute, "JSR", 6), Instruction::new(AddressingMode::IndirectX, "AND", 6), Instruction::new(AddressingMode::NoneAddressing, "XXX", 2), Instruction::new(AddressingMode::IndirectX, "RLA", 8), Instruction::new(AddressingMode::ZeroPage, "BIT", 3), Instruction::new(AddressingMode::ZeroPage, "AND", 3), Instruction::new(AddressingMode::ZeroPage, "ROL", 5), Instruction::new(AddressingMode::ZeroPage, "RLA", 5), Instruction::new(AddressingMode::NoneAddressing, "PLP", 4), Instruction::new(AddressingMode::Immediate, "AND", 2), Instruction::new(AddressingMode::Accumulator, "ROL", 2), Instruction::new(AddressingMode::Immediate, "ANC", 2), Instruction::new(AddressingMode::Absolute, "BIT", 4), Instruction::new(AddressingMode::Absolute, "AND", 4), Instruction::new(AddressingMode::Absolute, "ROL", 6), Instruction::new(AddressingMode::Absolute, "RLA", 6),
    Instruction::new(AddressingMode::NoneAddressing, "BMI", 2), Instruction::new(AddressingMode::IndirectY, "AND", 5), Instruction::new(AddressingMode::NoneAddressing, "XXX", 2), Instruction::new(AddressingMode::IndirectY, "RLA", 8), Instruction::new(AddressingMode::ZeroPageX, "NOP", 4), Instruction::new(AddressingMode::ZeroPageX, "AND", 4), Instruction::new(AddressingMode::ZeroPageX, "ROL", 6), Instruction::new(AddressingMode::ZeroPageX, "RLA", 6), Instruction::new(AddressingMode::NoneAddressing, "SEC", 2), Instruction::new(AddressingMode::AbsoluteY, "AND", 4), Instruction::new(AddressingMode::NoneAddressing, "NOP", 2), Instruction::new(AddressingMode::AbsoluteY, "RLA", 7), Instruction::new(AddressingMode::AbsoluteX, "IGN", 4), Instruction::new(AddressingMode::AbsoluteX, "AND", 4), Instruction::new(AddressingMode::AbsoluteX, "ROL", 7), Instruction::new(AddressingMode::AbsoluteX, "RLA", 7),
    Instruction::new(AddressingMode::NoneAddressing, "RTI", 6), Instruction::new(AddressingMode::IndirectX, "EOR", 6), Instruction::new(AddressingMode::NoneAddressing, "XXX", 2), Instruction::new(AddressingMode::IndirectX, "SRE", 8), Instruction::new(AddressingMode::ZeroPage, "NOP", 3), Instruction::new(AddressingMode::ZeroPage, "EOR", 3), Instruction::new(AddressingMode::ZeroPage, "LSR", 5), Instruction::new(AddressingMode::ZeroPage, "SRE", 5), Instruction::new(AddressingMode::NoneAddressing, "PHA", 3), Instruction::new(AddressingMode::Immediate, "EOR", 2), Instruction::new(AddressingMode::Accumulator, "LSR", 2), Instruction::new(AddressingMode::Immediate, "ALR", 2), Instruction::new(AddressingMode::Absolute, "JMP", 3), Instruction::new(AddressingMode::Absolute, "EOR", 4), Instruction::new(AddressingMode::Absolute, "LSR", 6), Instruction::new(AddressingMode::Absolute, "SRE", 6),
    Instruction::new(AddressingMode::NoneAddressing, "BVC", 2), Instruction::new(AddressingMode::IndirectY, "EOR", 5), Instruction::new(AddressingMode::NoneAddressing, "XXX", 2), Instruction::new(AddressingMode::IndirectY, "SRE", 8), Instruction::new(AddressingMode::ZeroPageX, "NOP", 4), Instruction::new(AddressingMode::ZeroPageX, "EOR", 4), Instruction::new(AddressingMode::ZeroPageX, "LSR", 6), Instruction::new(AddressingMode::ZeroPageX, "SRE", 6), Instruction::new(AddressingMode::NoneAddressing, "CLI", 2), Instruction::new(AddressingMode::AbsoluteY, "EOR", 4), Instruction::new(AddressingMode::NoneAddressing, "NOP", 2), Instruction::new(AddressingMode::AbsoluteY, "SRE", 7), Instruction::new(AddressingMode::AbsoluteX, "IGN", 4), Instruction::new(AddressingMode::AbsoluteX, "EOR", 4), Instruction::new(AddressingMode::AbsoluteX, "LSR", 7), Instruction::new(AddressingMode::AbsoluteX, "SRE", 7),
    Instruction::new(AddressingMode::NoneAddressing, "RTS", 6), Instruction::new(AddressingMode::IndirectX, "ADC", 6), Instruction::new(AddressingMode::NoneAddressing, "XXX", 2), Instruction::new(AddressingMode::IndirectX, "RRA", 8), Instruction::new(AddressingMode::ZeroPage, "NOP", 3), Instruction::new(AddressingMode::ZeroPage, "ADC", 3), Instruction::new(AddressingMode::ZeroPage, "ROR", 5), Instruction::new(AddressingMode::ZeroPage, "RRA", 5), Instruction::new(AddressingMode::NoneAddressing, "PLA", 4), Instruction::new(AddressingMode::Immediate, "ADC", 2), Instruction::new(AddressingMode::Accumulator, "ROR", 2), Instruction::new(AddressingMode::Immediate, "ARR", 2), Instruction::new(AddressingMode::NoneAddressing, "JMP", 5), Instruction::new(AddressingMode::Absolute, "ADC", 4), Instruction::new(AddressingMode::Absolute, "ROR", 6), Instruction::new(AddressingMode::Absolute, "RRA", 6),
    Instruction::new(AddressingMode::NoneAddressing, "BVS", 2), Instruction::new(AddressingMode::IndirectY, "ADC", 5), Instruction::new(AddressingMode::NoneAddressing, "XXX", 2), Instruction::new(AddressingMode::IndirectY, "RRA", 8), Instruction::new(AddressingMode::ZeroPageX, "NOP", 4), Instruction::new(AddressingMode::ZeroPageX, "ADC", 4), Instruction::new(AddressingMode::ZeroPageX, "ROR", 6), Instruction::new(AddressingMode::ZeroPageX, "RRA", 6), Instruction::new(AddressingMode::NoneAddressing, "SEI", 2), Instruction::new(AddressingMode::AbsoluteY, "ADC", 4), Instruction::new(AddressingMode::NoneAddressing, "NOP", 2), Instruction::new(AddressingMode::AbsoluteY, "RRA", 7), Instruction::new(AddressingMode::AbsoluteX, "IGN", 4), Instruction::new(AddressingMode::AbsoluteX, "ADC", 4), Instruction::new(AddressingMode::AbsoluteX, "ROR", 7), Instruction::new(AddressingMode::AbsoluteX, "RRA", 7),
    Instruction::new(AddressingMode::Immediate, "SKB", 2), Instruction::new(AddressingMode::IndirectX, "STA", 6), Instruction::new(AddressingMode::Immediate, "SKB", 2), Instruction::new(AddressingMode::IndirectX, "SAX", 6), Instruction::new(AddressingMode::ZeroPage, "STY", 3), Instruction::new(AddressingMode::ZeroPage, "STA", 3), Instruction::new(AddressingMode::ZeroPage, "STX", 3), Instruction::new(AddressingMode::ZeroPage, "SAX", 3), Instruction::new(AddressingMode::NoneAddressing, "DEY", 2), Instruction::new(AddressingMode::Immediate, "SKB", 2), Instruction::new(AddressingMode::NoneAddressing, "TXA", 2), Instruction::new(AddressingMode::Immediate, "XAA", 2), Instruction::new(AddressingMode::Absolute, "STY", 4), Instruction::new(AddressingMode::Absolute, "STA", 4), Instruction::new(AddressingMode::Absolute, "STX", 4), Instruction::new(AddressingMode::Absolute, "SAX", 4),
    Instruction::new(AddressingMode::NoneAddressing, "BCC", 2), Instruction::new(AddressingMode::IndirectY, "STA", 6), Instruction::new(AddressingMode::NoneAddressing, "XXX", 2), Instruction::new(AddressingMode::IndirectY, "AHX", 6), Instruction::new(AddressingMode::ZeroPageX, "STY", 4), Instruction::new(AddressingMode::ZeroPageX, "STA", 4), Instruction::new(AddressingMode::ZeroPageY, "STX", 4), Instruction::new(AddressingMode::ZeroPageY, "SAX", 4), Instruction::new(AddressingMode::NoneAddressing, "TYA", 2), Instruction::new(AddressingMode::AbsoluteY, "STA", 5), Instruction::new(AddressingMode::NoneAddressing, "TXS", 2), Instruction::new(AddressingMode::AbsoluteY, "TAS", 5), Instruction::new(AddressingMode::AbsoluteX, "SYA", 5), Instruction::new(AddressingMode::AbsoluteX, "STA", 5), Instruction::new(AddressingMode::AbsoluteY, "SXA", 5), Instruction::new(AddressingMode::AbsoluteY, "AHX", 5),
    Instruction::new(AddressingMode::Immediate, "LDY", 2), Instruction::new(AddressingMode::IndirectX, "LDA", 6), Instruction::new(AddressingMode::Immediate, "LDX", 2), Instruction::new(AddressingMode::IndirectX, "LAX", 6), Instruction::new(AddressingMode::ZeroPage, "LDY", 3), Instruction::new(AddressingMode::ZeroPage, "LDA", 3), Instruction::new(AddressingMode::ZeroPage, "LDX", 3), Instruction::new(AddressingMode::ZeroPage, "LAX", 3), Instruction::new(AddressingMode::NoneAddressing, "TAY", 2), Instruction::new(AddressingMode::Immediate, "LDA", 2), Instruction::new(AddressingMode::NoneAddressing, "TAX", 2), Instruction::new(AddressingMode::Immediate, "LAX", 2), Instruction::new(AddressingMode::Absolute, "LDY", 4), Instruction::new(AddressingMode::Absolute, "LDA", 4), Instruction::new(AddressingMode::Absolute, "LDX", 4), Instruction::new(AddressingMode::Absolute, "LAX", 4),
    Instruction::new(AddressingMode::NoneAddressing, "BCS", 2), Instruction::new(AddressingMode::IndirectY, "LDA", 5), Instruction::new(AddressingMode::NoneAddressing, "XXX", 2), Instruction::new(AddressingMode::IndirectY, "LAX", 5), Instruction::new(AddressingMode::ZeroPageX, "LDY", 4), Instruction::new(AddressingMode::ZeroPageX, "LDA", 4), Instruction::new(AddressingMode::ZeroPageY, "LDX", 4), Instruction::new(AddressingMode::ZeroPageY, "LAX", 4), Instruction::new(AddressingMode::NoneAddressing, "CLV", 2), Instruction::new(AddressingMode::AbsoluteY, "LDA", 4), Instruction::new(AddressingMode::NoneAddressing, "TSX", 2), Instruction::new(AddressingMode::AbsoluteY, "LAS", 4), Instruction::new(AddressingMode::AbsoluteX, "LDY", 4), Instruction::new(AddressingMode::AbsoluteX, "LDA", 4), Instruction::new(AddressingMode::AbsoluteY, "LDX", 4), Instruction::new(AddressingMode::AbsoluteY, "LAX", 4),
    Instruction::new(AddressingMode::Immediate, "CPY", 2), Instruction::new(AddressingMode::IndirectX, "CMP", 6), Instruction::new(AddressingMode::Immediate, "SKB", 2), Instruction::new(AddressingMode::IndirectX, "DCP", 8), Instruction::new(AddressingMode::ZeroPage, "CPY", 3), Instruction::new(AddressingMode::ZeroPage, "CMP", 3), Instruction::new(AddressingMode::ZeroPage, "DEC", 5), Instruction::new(AddressingMode::ZeroPage, "DCP", 5), Instruction::new(AddressingMode::NoneAddressing, "INY", 2), Instruction::new(AddressingMode::Immediate, "CMP", 2), Instruction::new(AddressingMode::NoneAddressing, "DEX", 2), Instruction::new(AddressingMode::Immediate, "AXS", 2), Instruction::new(AddressingMode::Absolute, "CPY", 4), Instruction::new(AddressingMode::Absolute, "CMP", 4), Instruction::new(AddressingMode::Absolute, "DEC", 6), Instruction::new(AddressingMode::Absolute, "DCP", 6),
    Instruction::new(AddressingMode::NoneAddressing, "BNE", 2), Instruction::new(AddressingMode::IndirectY, "CMP", 5), Instruction::new(AddressingMode::NoneAddressing, "XXX", 2), Instruction::new(AddressingMode::IndirectY, "DCP", 8), Instruction::new(AddressingMode::ZeroPageX, "NOP", 4), Instruction::new(AddressingMode::ZeroPageX, "CMP", 4), Instruction::new(AddressingMode::ZeroPageX, "DEC", 6), Instruction::new(AddressingMode::ZeroPageX, "DCP", 6), Instruction::new(AddressingMode::NoneAddressing, "CLD", 2), Instruction::new(AddressingMode::AbsoluteY, "CMP", 4), Instruction::new(AddressingMode::NoneAddressing, "NOP", 2), Instruction::new(AddressingMode::AbsoluteY, "DCP", 7), Instruction::new(AddressingMode::AbsoluteX, "IGN", 4), Instruction::new(AddressingMode::AbsoluteX, "CMP", 4), Instruction::new(AddressingMode::AbsoluteX, "DEC", 7), Instruction::new(AddressingMode::AbsoluteX, "DCP", 7),
    Instruction::new(AddressingMode::Immediate, "CPX", 2), Instruction::new(AddressingMode::IndirectX, "SBC", 6), Instruction::new(AddressingMode::Immediate, "SKB", 2), Instruction::new(AddressingMode::IndirectX, "ISB", 8), Instruction::new(AddressingMode::ZeroPage, "CPX", 3), Instruction::new(AddressingMode::ZeroPage, "SBC", 3), Instruction::new(AddressingMode::ZeroPage, "INC", 5), Instruction::new(AddressingMode::ZeroPage, "ISB", 5), Instruction::new(AddressingMode::NoneAddressing, "INX", 2), Instruction::new(AddressingMode::Immediate, "SBC", 2), Instruction::new(AddressingMode::NoneAddressing, "NOP", 2), Instruction::new(AddressingMode::Immediate, "SBC", 2), Instruction::new(AddressingMode::Absolute, "CPX", 4), Instruction::new(AddressingMode::Absolute, "SBC", 4), Instruction::new(AddressingMode::Absolute, "INC", 6), Instruction::new(AddressingMode::Absolute, "ISB", 6),
    Instruction::new(AddressingMode::NoneAddressing, "BEQ", 2), Instruction::new(AddressingMode::IndirectY, "SBC", 5), Instruction::new(AddressingMode::NoneAddressing, "XXX", 2), Instruction::new(AddressingMode::IndirectY, "ISB", 8), Instruction::new(AddressingMode::ZeroPageX, "NOP", 4), Instruction::new(AddressingMode::ZeroPageX, "SBC", 4), Instruction::new(AddressingMode::ZeroPageX, "INC", 6), Instruction::new(AddressingMode::ZeroPageX, "ISB", 6), Instruction::new(AddressingMode::NoneAddressing, "SED", 2), Instruction::new(AddressingMode::AbsoluteY, "SBC", 4), Instruction::new(AddressingMode::NoneAddressing, "NOP", 2), Instruction::new(AddressingMode::AbsoluteY, "ISB", 7), Instruction::new(AddressingMode::AbsoluteX, "IGN", 4), Instruction::new(AddressingMode::AbsoluteX, "SBC", 4), Instruction::new(AddressingMode::AbsoluteX, "INC", 7), Instruction::new(AddressingMode::AbsoluteX, "ISB", 7),
  ];
}


impl CPU {
  pub fn decode(&mut self, op_code: u8) {
    let instruction = &INSTRUCTIONS[op_code as usize];

    let instruction_name = instruction.name;
    let mode = &instruction.mode;
    let instr_address = format!("{:X}", self.registers.pc - 1);

    println!("found instruction {instruction_name} with mode {mode} at address {instr_address}");

    match instruction.name {
      "ADC" => self.adc(mode),
      "AND" => self.and(mode),
      "ASL" => self.asl(mode),
      "BCC" => self.branch(!self.registers.p.contains(CpuFlags::CARRY)),
      "BCS" => self.branch(self.registers.p.contains(CpuFlags::CARRY)),
      "BEQ" => self.branch(self.registers.p.contains(CpuFlags::ZERO)),
      "BIT" => self.bit(mode),
      "BMI" => self.branch(self.registers.p.contains(CpuFlags::NEGATIVE)),
      "BNE" => self.branch(!self.registers.p.contains(CpuFlags::ZERO)),
      "BPL" => self.branch(!self.registers.p.contains(CpuFlags::NEGATIVE)),
      "BRK" => self.brk(),
      "BVC" => self.branch(!self.registers.p.contains(CpuFlags::OVERFLOW)),
      "BVS" => self.branch(self.registers.p.contains(CpuFlags::OVERFLOW)),
      "CLC" => self.registers.p.remove(CpuFlags::CARRY),
      "CLD" => self.registers.p.remove(CpuFlags::DECIMAL_MODE),
      "CLI" => self.registers.p.remove(CpuFlags::INTERRUPT_DISABLE),
      "CLV" => self.registers.p.remove(CpuFlags::OVERFLOW),
      "CMP" => self.compare(mode, self.registers.a),
      "CPX" => self.compare(mode, self.registers.x),
      "CPY" => self.compare(mode, self.registers.y),
      "DEC" => self.dec(mode),
      "DEX" => self.dex(),
      "DEY" => self.dey(),
      "EOR" => self.eor(mode),
      "INC" => self.inc(mode),
      "INX" => self.inx(),
      "INY" => self.iny(),
      "LDA" => self.lda(mode),
      "TAX" => self.tax(),
      "TAY" => self.tay(),
      _ => println!("instruction not implemented yet")
    }

    self.cycle(instruction.cycles);
  }

  fn compare(&mut self, mode: &AddressingMode, compare_to: u8) {
    let address = self.get_operand_address(mode);

    let val = self.mem_read(address);

    let result = compare_to - val;

    self.registers.p.set(CpuFlags::CARRY, result > 0);

    self.set_zero_and_negative_flags(result);
  }

  fn inc(&mut self, mode: &AddressingMode) {
    let address = self.get_operand_address(mode);

    let result = self.mem_read(address) + 1;

    self.set_zero_and_negative_flags(result);

    self.mem_write(address, result);
  }

  fn eor(&mut self, mode: &AddressingMode) {
    let address = self.get_operand_address(mode);

    let val = self.mem_read(address);

    self.registers.a ^= val;

    self.set_zero_and_negative_flags(self.registers.a);
  }

  fn inx(&mut self) {
    self.registers.x += 1;

    self.set_zero_and_negative_flags(self.registers.x);
  }

  fn iny(&mut self) {
    self.registers.y += 1;

    self.set_zero_and_negative_flags(self.registers.y);
  }

  fn bit(&mut self, mode: &AddressingMode) {
    let address = self.get_operand_address(mode);

    let val = self.mem_read(address);

    let result = val & self.registers.a;

    self.registers.p.set(CpuFlags::ZERO, result == 0);
    self.registers.p.set(CpuFlags::OVERFLOW, (val >> 6) & 0b1 == 1);
    self.registers.p.set(CpuFlags::NEGATIVE, val >> 7 == 1);
  }

  fn asl_accumulator(&mut self) {
    self.registers.p.set(CpuFlags::CARRY, self.registers.a >> 7 == 1);

    self.registers.a = self.registers.a << 1;

    self.set_zero_and_negative_flags(self.registers.a);
  }

  fn asl(&mut self, mode: &AddressingMode) {
    if matches!(mode, AddressingMode::Accumulator) {
      return self.asl_accumulator();
    }

    let address = self.get_operand_address(mode);

    let mut val = self.mem_read(address);

    self.registers.p.set(CpuFlags::CARRY, val >> 7 == 1);

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

  fn and(&mut self, mode: &AddressingMode) {
    let address = self.get_operand_address(mode);

    let val = self.mem_read(address);

    self.registers.a &= val;

    self.set_zero_and_negative_flags(self.registers.a);
  }

  fn dec(&mut self, mode: &AddressingMode) {
    let address = self.get_operand_address(mode);

    let result = self.mem_read(address) - 1;

    self.mem_write(address, result);

    self.set_zero_and_negative_flags(result);
  }

  fn dex(&mut self) {
    self.registers.x -= 1;

    self.set_zero_and_negative_flags(self.registers.x);
  }

  fn dey(&mut self) {
    self.registers.y -= 1;

    self.set_zero_and_negative_flags(self.registers.y);
  }

  fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
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
        let address = self.get_absolute_offset_address(self.registers.x);

        self.registers.pc += 2;

        address
      }
      AddressingMode::AbsoluteY => {
        let address = self.get_absolute_offset_address(self.registers.y);

        self.registers.pc += 2;

        address
      }
      AddressingMode::Indirect => {
        let indirect_address = self.mem_read_u16(self.registers.pc);

        // per https://github.com/bugzmanov/nes_ebook/blob/master/code/ch8/src/cpu.rs
        // if the address ends in ff there is a bug where the lower byte of the address
        // wraps around and starts at 00. (ie: if address is 0x30ff, upper byte is at 0x3000).
        // Otherwise it works exactly as it should

        let address = if indirect_address & 0xff == 0xff   {
          let lower_byte = self.mem_read(indirect_address) as u16;
          let upper_byte = self.mem_read(indirect_address & 0xff00) as u16;

          (upper_byte << 8) | lower_byte
        } else {
          self.mem_read_u16(indirect_address as u16)
        };

        address
      }
      AddressingMode::IndirectX => {
        let base_address = self.mem_read(self.registers.pc);

        let actual_address = base_address.wrapping_add(self.registers.x);

        let low_byte = self.mem_read(actual_address as u16) as u16;
        let high_byte = self.mem_read(actual_address.wrapping_add(1) as u16) as u16;

        self.registers.pc += 1;

        (high_byte << 8) | low_byte
      }
      AddressingMode::IndirectY => {
         let base_address = self.mem_read(self.registers.pc);

         let low_byte = self.mem_read(base_address as u16) as u16;
         let high_byte = self.mem_read(base_address.wrapping_add(1) as u16) as u16;

         let actual_address = ((high_byte << 8) | low_byte).wrapping_add(self.registers.y as u16);

         self.registers.pc += 1;

         actual_address
      }
      AddressingMode::NoneAddressing => panic!("mode is not supported"),
      AddressingMode::Accumulator => panic!("no address required for this mode")
    }
  }

  fn get_absolute_offset_address(&self, offset: u8) -> u16 {
    let base_address = self.mem_read_u16(self.registers.pc);

    base_address.wrapping_add(offset as u16)
  }

  fn lda(&mut self, mode: &AddressingMode) {
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

  fn adc(&mut self, mode: &AddressingMode) {
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
}