pub mod instruction;

use crate::nes::CPU;
use crate::nes::cpu::CpuFlags;

use instruction::AddressingMode;

use instruction::Opname::{
  ADC,
  AHX,
  ALR,
  ANC,
  AND,
  ARR,
  ASL,
  AXS,
  BCC,
  BCS,
  BEQ,
  BIT,
  BMI,
  BNE,
  BPL,
  BRK,
  BVC,
  BVS,
  CLC,
  CLD,
  CLI,
  CLV,
  CMP,
  CPX,
  CPY,
  DCP,
  DEC,
  DEX,
  DEY,
  EOR,
  IGN,
  INC,
  INX,
  INY,
  ISC,
  JMP,
  JSR,
  LAS,
  LAX,
  LDA,
  LDX,
  LDY,
  LSR,
  NOP,
  ORA,
  PHA,
  PHP,
  PLA,
  PLP,
  RLA,
  ROL,
  ROR,
  RRA,
  RTI,
  RTS,
  SAX,
  SBC,
  SEC,
  SED,
  SEI,
  SKB,
  SLO,
  SRE,
  STA,
  STX,
  STY,
  SXA,
  SYA,
  TAS,
  TAX,
  TAY,
  TSX,
  TXA,
  TXS,
  TYA,
  XAA,
  XXX,
};

use self::instruction::Opname;

pub struct Instruction {
  _code: u8,
  mode: AddressingMode,
  name: Opname,
  cycles: u16
}

impl Instruction {
  pub fn new(code: u8, mode: AddressingMode, name: Opname, cycles: u16) -> Self {
    Instruction {
      _code: code,
      mode,
      name,
      cycles
    }
  }
}
lazy_static! {
  pub static ref INSTRUCTIONS: Vec<Instruction> = vec![
    Instruction::new(0x00, AddressingMode::Immediate, BRK, 7), Instruction::new(0x01, AddressingMode::IndirectX, ORA, 6), Instruction::new(0x02, AddressingMode::NoneAddressing, XXX, 2), Instruction::new(0x03, AddressingMode::IndirectX, SLO, 8), Instruction::new(0x04, AddressingMode::ZeroPage, NOP, 3), Instruction::new(0x05, AddressingMode::ZeroPage, ORA, 3), Instruction::new(0x06, AddressingMode::ZeroPage, ASL, 5), Instruction::new(0x07, AddressingMode::ZeroPage, SLO, 5), Instruction::new(0x08, AddressingMode::NoneAddressing, PHP, 3), Instruction::new(0x09, AddressingMode::Immediate, ORA, 2), Instruction::new(0x0A, AddressingMode::Accumulator, ASL, 2), Instruction::new(0x0B, AddressingMode::Immediate, ANC, 2), Instruction::new(0x0C, AddressingMode::Absolute, NOP, 4), Instruction::new(0x0D, AddressingMode::Absolute, ORA, 4), Instruction::new(0x0E, AddressingMode::Absolute, ASL, 6), Instruction::new(0x0F, AddressingMode::Absolute, SLO, 6),
    Instruction::new(0x10, AddressingMode::NoneAddressing, BPL, 2), Instruction::new(0x11, AddressingMode::IndirectY, ORA, 5), Instruction::new(0x12, AddressingMode::NoneAddressing, XXX, 2), Instruction::new(0x13, AddressingMode::IndirectY, SLO, 8), Instruction::new(0x14, AddressingMode::ZeroPageX, NOP, 4), Instruction::new(0x15, AddressingMode::ZeroPageX, ORA, 4), Instruction::new(0x16, AddressingMode::ZeroPageX, ASL, 6), Instruction::new(0x17, AddressingMode::ZeroPageX, SLO, 6), Instruction::new(0x18, AddressingMode::NoneAddressing, CLC, 2), Instruction::new(0x19, AddressingMode::AbsoluteY, ORA, 4), Instruction::new(0x1A, AddressingMode::NoneAddressing, NOP, 2), Instruction::new(0x1B, AddressingMode::AbsoluteY, SLO, 7), Instruction::new(0x1C, AddressingMode::AbsoluteX, IGN, 4), Instruction::new(0x1D, AddressingMode::AbsoluteX, ORA, 4), Instruction::new(0x1E, AddressingMode::AbsoluteX, ASL, 7), Instruction::new(0x1F, AddressingMode::AbsoluteX, SLO, 7),
    Instruction::new(0x20, AddressingMode::Absolute, JSR, 6), Instruction::new(0x21, AddressingMode::IndirectX, AND, 6), Instruction::new(0x22, AddressingMode::NoneAddressing, XXX, 2), Instruction::new(0x23, AddressingMode::IndirectX, RLA, 8), Instruction::new(0x24, AddressingMode::ZeroPage, BIT, 3), Instruction::new(0x25, AddressingMode::ZeroPage, AND, 3), Instruction::new(0x26, AddressingMode::ZeroPage, ROL, 5), Instruction::new(0x27, AddressingMode::ZeroPage, RLA, 5), Instruction::new(0x28, AddressingMode::NoneAddressing, PLP, 4), Instruction::new(0x29, AddressingMode::Immediate, AND, 2), Instruction::new(0x2A, AddressingMode::Accumulator, ROL, 2), Instruction::new(0x2B, AddressingMode::Immediate, ANC, 2), Instruction::new(0x2C, AddressingMode::Absolute, BIT, 4), Instruction::new(0x2D, AddressingMode::Absolute, AND, 4), Instruction::new(0x2E, AddressingMode::Absolute, ROL, 6), Instruction::new(0x2F, AddressingMode::Absolute, RLA, 6),
    Instruction::new(0x30, AddressingMode::NoneAddressing, BMI, 2), Instruction::new(0x31, AddressingMode::IndirectY, AND, 5), Instruction::new(0x32, AddressingMode::NoneAddressing, XXX, 2), Instruction::new(0x33, AddressingMode::IndirectY, RLA, 8), Instruction::new(0x34, AddressingMode::ZeroPageX, NOP, 4), Instruction::new(0x35, AddressingMode::ZeroPageX, AND, 4), Instruction::new(0x36, AddressingMode::ZeroPageX, ROL, 6), Instruction::new(0x37, AddressingMode::ZeroPageX, RLA, 6), Instruction::new(0x38, AddressingMode::NoneAddressing, SEC, 2), Instruction::new(0x39, AddressingMode::AbsoluteY, AND, 4), Instruction::new(0x3A, AddressingMode::NoneAddressing, NOP, 2), Instruction::new(0x3B, AddressingMode::AbsoluteY, RLA, 7), Instruction::new(0x3C, AddressingMode::AbsoluteX, IGN, 4), Instruction::new(0x3D, AddressingMode::AbsoluteX, AND, 4), Instruction::new(0x3E, AddressingMode::AbsoluteX, ROL, 7), Instruction::new(0x3F, AddressingMode::AbsoluteX, RLA, 7),
    Instruction::new(0x40, AddressingMode::NoneAddressing, RTI, 6), Instruction::new(0x41, AddressingMode::IndirectX, EOR, 6), Instruction::new(0x42, AddressingMode::NoneAddressing, XXX, 2), Instruction::new(0x43, AddressingMode::IndirectX, SRE, 8), Instruction::new(0x44, AddressingMode::ZeroPage, NOP, 3), Instruction::new(0x45, AddressingMode::ZeroPage, EOR, 3), Instruction::new(0x46, AddressingMode::ZeroPage, LSR, 5), Instruction::new(0x47, AddressingMode::ZeroPage, SRE, 5), Instruction::new(0x48, AddressingMode::NoneAddressing, PHA, 3), Instruction::new(0x49, AddressingMode::Immediate, EOR, 2), Instruction::new(0x4A, AddressingMode::Accumulator, LSR, 2), Instruction::new(0x4B, AddressingMode::Immediate, ALR, 2), Instruction::new(0x4C, AddressingMode::Absolute, JMP, 3), Instruction::new(0x4D, AddressingMode::Absolute, EOR, 4), Instruction::new(0x4E, AddressingMode::Absolute, LSR, 6), Instruction::new(0x4F, AddressingMode::Absolute, SRE, 6),
    Instruction::new(0x50, AddressingMode::NoneAddressing, BVC, 2), Instruction::new(0x51, AddressingMode::IndirectY, EOR, 5), Instruction::new(0x52, AddressingMode::NoneAddressing, XXX, 2), Instruction::new(0x53, AddressingMode::IndirectY, SRE, 8), Instruction::new(0x54, AddressingMode::ZeroPageX, NOP, 4), Instruction::new(0x55, AddressingMode::ZeroPageX, EOR, 4), Instruction::new(0x56, AddressingMode::ZeroPageX, LSR, 6), Instruction::new(0x57, AddressingMode::ZeroPageX, SRE, 6), Instruction::new(0x58, AddressingMode::NoneAddressing, CLI, 2), Instruction::new(0x59, AddressingMode::AbsoluteY, EOR, 4), Instruction::new(0x5A, AddressingMode::NoneAddressing, NOP, 2), Instruction::new(0x5B, AddressingMode::AbsoluteY, SRE, 7), Instruction::new(0x5C, AddressingMode::AbsoluteX, IGN, 4), Instruction::new(0x5D, AddressingMode::AbsoluteX, EOR, 4), Instruction::new(0x5E, AddressingMode::AbsoluteX, LSR, 7), Instruction::new(0x5F, AddressingMode::AbsoluteX, SRE, 7),
    Instruction::new(0x60, AddressingMode::NoneAddressing, RTS, 6), Instruction::new(0x61, AddressingMode::IndirectX, ADC, 6), Instruction::new(0x62, AddressingMode::NoneAddressing, XXX, 2), Instruction::new(0x63, AddressingMode::IndirectX, RRA, 8), Instruction::new(0x64, AddressingMode::ZeroPage, NOP, 3), Instruction::new(0x65, AddressingMode::ZeroPage, ADC, 3), Instruction::new(0x66, AddressingMode::ZeroPage, ROR, 5), Instruction::new(0x67, AddressingMode::ZeroPage, RRA, 5), Instruction::new(0x68, AddressingMode::NoneAddressing, PLA, 4), Instruction::new(0x69, AddressingMode::Immediate, ADC, 2), Instruction::new(0x6A, AddressingMode::Accumulator, ROR, 2), Instruction::new(0x6B, AddressingMode::Immediate, ARR, 2), Instruction::new(0x6C, AddressingMode::Indirect, JMP, 5), Instruction::new(0x6D, AddressingMode::Absolute, ADC, 4), Instruction::new(0x6E, AddressingMode::Absolute, ROR, 6), Instruction::new(0x6F, AddressingMode::Absolute, RRA, 6),
    Instruction::new(0x70, AddressingMode::NoneAddressing, BVS, 2), Instruction::new(0x71, AddressingMode::IndirectY, ADC, 5), Instruction::new(0x72, AddressingMode::NoneAddressing, XXX, 2), Instruction::new(0x73, AddressingMode::IndirectY, RRA, 8), Instruction::new(0x74, AddressingMode::ZeroPageX, NOP, 4), Instruction::new(0x75, AddressingMode::ZeroPageX, ADC, 4), Instruction::new(0x76, AddressingMode::ZeroPageX, ROR, 6), Instruction::new(0x77, AddressingMode::ZeroPageX, RRA, 6), Instruction::new(0x78, AddressingMode::NoneAddressing, SEI, 2), Instruction::new(0x79, AddressingMode::AbsoluteY, ADC, 4), Instruction::new(0x7A, AddressingMode::NoneAddressing, NOP, 2), Instruction::new(0x7B, AddressingMode::AbsoluteY, RRA, 7), Instruction::new(0x7C, AddressingMode::AbsoluteX, IGN, 4), Instruction::new(0x7D, AddressingMode::AbsoluteX, ADC, 4), Instruction::new(0x7E, AddressingMode::AbsoluteX, ROR, 7), Instruction::new(0x7F, AddressingMode::AbsoluteX, RRA, 7),
    Instruction::new(0x80, AddressingMode::Immediate, SKB, 2), Instruction::new(0x81, AddressingMode::IndirectX, STA, 6), Instruction::new(0x82, AddressingMode::Immediate, SKB, 2), Instruction::new(0x83, AddressingMode::IndirectX, SAX, 6), Instruction::new(0x84, AddressingMode::ZeroPage, STY, 3), Instruction::new(0x85, AddressingMode::ZeroPage, STA, 3), Instruction::new(0x86, AddressingMode::ZeroPage, STX, 3), Instruction::new(0x87, AddressingMode::ZeroPage, SAX, 3), Instruction::new(0x88, AddressingMode::NoneAddressing, DEY, 2), Instruction::new(0x89, AddressingMode::Immediate, SKB, 2), Instruction::new(0x8A, AddressingMode::NoneAddressing, TXA, 2), Instruction::new(0x8B, AddressingMode::Immediate, XAA, 2), Instruction::new(0x8C, AddressingMode::Absolute, STY, 4), Instruction::new(0x8D, AddressingMode::Absolute, STA, 4), Instruction::new(0x8E, AddressingMode::Absolute, STX, 4), Instruction::new(0x8F, AddressingMode::Absolute, SAX, 4),
    Instruction::new(0x90, AddressingMode::NoneAddressing, BCC, 2), Instruction::new(0x91, AddressingMode::IndirectY, STA, 6), Instruction::new(0x92, AddressingMode::NoneAddressing, XXX, 2), Instruction::new(0x93, AddressingMode::IndirectY, AHX, 6), Instruction::new(0x94, AddressingMode::ZeroPageX, STY, 4), Instruction::new(0x95, AddressingMode::ZeroPageX, STA, 4), Instruction::new(0x96, AddressingMode::ZeroPageY, STX, 4), Instruction::new(0x97, AddressingMode::ZeroPageY, SAX, 4), Instruction::new(0x98, AddressingMode::NoneAddressing, TYA, 2), Instruction::new(0x99, AddressingMode::AbsoluteY, STA, 5), Instruction::new(0x9A, AddressingMode::NoneAddressing, TXS, 2), Instruction::new(0x9B, AddressingMode::AbsoluteY, TAS, 5), Instruction::new(0x9C, AddressingMode::AbsoluteX, SYA, 5), Instruction::new(0x9D, AddressingMode::AbsoluteX, STA, 5), Instruction::new(0x9E, AddressingMode::AbsoluteY, SXA, 5), Instruction::new(0x9F, AddressingMode::AbsoluteY, AHX, 5),
    Instruction::new(0xA0, AddressingMode::Immediate, LDY, 2), Instruction::new(0xA1, AddressingMode::IndirectX, LDA, 6), Instruction::new(0xA2, AddressingMode::Immediate, LDX, 2), Instruction::new(0xA3, AddressingMode::IndirectX, LAX, 6), Instruction::new(0xA4, AddressingMode::ZeroPage, LDY, 3), Instruction::new(0xA5, AddressingMode::ZeroPage, LDA, 3), Instruction::new(0xA6, AddressingMode::ZeroPage, LDX, 3), Instruction::new(0xA7, AddressingMode::ZeroPage, LAX, 3), Instruction::new(0xA8, AddressingMode::NoneAddressing, TAY, 2), Instruction::new(0xA9, AddressingMode::Immediate, LDA, 2), Instruction::new(0xAA, AddressingMode::NoneAddressing, TAX, 2), Instruction::new(0xAB, AddressingMode::Immediate, LAX, 2), Instruction::new(0xAC, AddressingMode::Absolute, LDY, 4), Instruction::new(0xAD, AddressingMode::Absolute, LDA, 4), Instruction::new(0xAE, AddressingMode::Absolute, LDX, 4), Instruction::new(0xAF, AddressingMode::Absolute, LAX, 4),
    Instruction::new(0xB0, AddressingMode::NoneAddressing, BCS, 2), Instruction::new(0xB1, AddressingMode::IndirectY, LDA, 5), Instruction::new(0xB2, AddressingMode::NoneAddressing, XXX, 2), Instruction::new(0xB3, AddressingMode::IndirectY, LAX, 5), Instruction::new(0xB4, AddressingMode::ZeroPageX, LDY, 4), Instruction::new(0xB5, AddressingMode::ZeroPageX, LDA, 4), Instruction::new(0xB6, AddressingMode::ZeroPageY, LDX, 4), Instruction::new(0xB7, AddressingMode::ZeroPageY, LAX, 4), Instruction::new(0xB8, AddressingMode::NoneAddressing, CLV, 2), Instruction::new(0xB9, AddressingMode::AbsoluteY, LDA, 4), Instruction::new(0xBA, AddressingMode::NoneAddressing, TSX, 2), Instruction::new(0xBB, AddressingMode::AbsoluteY, LAS, 4), Instruction::new(0xBC, AddressingMode::AbsoluteX, LDY, 4), Instruction::new(0xBD, AddressingMode::AbsoluteX, LDA, 4), Instruction::new(0xBE, AddressingMode::AbsoluteY, LDX, 4), Instruction::new(0xBF, AddressingMode::AbsoluteY, LAX, 4),
    Instruction::new(0xC0, AddressingMode::Immediate, CPY, 2), Instruction::new(0xC1, AddressingMode::IndirectX, CMP, 6), Instruction::new(0xC2, AddressingMode::Immediate, SKB, 2), Instruction::new(0xC3, AddressingMode::IndirectX, DCP, 8), Instruction::new(0xC4, AddressingMode::ZeroPage, CPY, 3), Instruction::new(0xC5, AddressingMode::ZeroPage, CMP, 3), Instruction::new(0xC6, AddressingMode::ZeroPage, DEC, 5), Instruction::new(0xC7, AddressingMode::ZeroPage, DCP, 5), Instruction::new(0xC8, AddressingMode::NoneAddressing, INY, 2), Instruction::new(0xC9, AddressingMode::Immediate, CMP, 2), Instruction::new(0xCA, AddressingMode::NoneAddressing, DEX, 2), Instruction::new(0xCB, AddressingMode::Immediate, AXS, 2), Instruction::new(0xCC, AddressingMode::Absolute, CPY, 4), Instruction::new(0xCD, AddressingMode::Absolute, CMP, 4), Instruction::new(0xCE, AddressingMode::Absolute, DEC, 6), Instruction::new(0xCF, AddressingMode::Absolute, DCP, 6),
    Instruction::new(0xD0, AddressingMode::NoneAddressing, BNE, 2), Instruction::new(0xD1, AddressingMode::IndirectY, CMP, 5), Instruction::new(0xD2, AddressingMode::NoneAddressing, XXX, 2), Instruction::new(0xD3, AddressingMode::IndirectY, DCP, 8), Instruction::new(0xD4, AddressingMode::ZeroPageX, NOP, 4), Instruction::new(0xD5, AddressingMode::ZeroPageX, CMP, 4), Instruction::new(0xD6, AddressingMode::ZeroPageX, DEC, 6), Instruction::new(0xD7, AddressingMode::ZeroPageX, DCP, 6), Instruction::new(0xD8, AddressingMode::NoneAddressing, CLD, 2), Instruction::new(0xD9, AddressingMode::AbsoluteY, CMP, 4), Instruction::new(0xDA, AddressingMode::NoneAddressing, NOP, 2), Instruction::new(0xDB, AddressingMode::AbsoluteY, DCP, 7), Instruction::new(0xDC, AddressingMode::AbsoluteX, IGN, 4), Instruction::new(0xDD, AddressingMode::AbsoluteX, CMP, 4), Instruction::new(0xDE, AddressingMode::AbsoluteX, DEC, 7), Instruction::new(0xDF, AddressingMode::AbsoluteX, DCP, 7),
    Instruction::new(0xE0, AddressingMode::Immediate, CPX, 2), Instruction::new(0xE1, AddressingMode::IndirectX, SBC, 6), Instruction::new(0xE2, AddressingMode::Immediate, SKB, 2), Instruction::new(0xE3, AddressingMode::IndirectX, ISC, 8), Instruction::new(0xE4, AddressingMode::ZeroPage, CPX, 3), Instruction::new(0xE5, AddressingMode::ZeroPage, SBC, 3), Instruction::new(0xE6, AddressingMode::ZeroPage, INC, 5), Instruction::new(0xE7, AddressingMode::ZeroPage, ISC, 5), Instruction::new(0xE8, AddressingMode::NoneAddressing, INX, 2), Instruction::new(0xE9, AddressingMode::Immediate, SBC, 2), Instruction::new(0xEA, AddressingMode::NoneAddressing, NOP, 2), Instruction::new(0xEB, AddressingMode::Immediate, SBC, 2), Instruction::new(0xEC, AddressingMode::Absolute, CPX, 4), Instruction::new(0xED, AddressingMode::Absolute, SBC, 4), Instruction::new(0xEE, AddressingMode::Absolute, INC, 6), Instruction::new(0xEF, AddressingMode::Absolute, ISC, 6),
    Instruction::new(0xF0, AddressingMode::NoneAddressing, BEQ, 2), Instruction::new(0xF1, AddressingMode::IndirectY, SBC, 5), Instruction::new(0xF2, AddressingMode::NoneAddressing, XXX, 2), Instruction::new(0xF3, AddressingMode::IndirectY, ISC, 8), Instruction::new(0xF4, AddressingMode::ZeroPageX, NOP, 4), Instruction::new(0xF5, AddressingMode::ZeroPageX, SBC, 4), Instruction::new(0xF6, AddressingMode::ZeroPageX, INC, 6), Instruction::new(0xF7, AddressingMode::ZeroPageX, ISC, 6), Instruction::new(0xF8, AddressingMode::NoneAddressing, SED, 2), Instruction::new(0xF9, AddressingMode::AbsoluteY, SBC, 4), Instruction::new(0xFA, AddressingMode::NoneAddressing, NOP, 2), Instruction::new(0xFB, AddressingMode::AbsoluteY, ISC, 7), Instruction::new(0xFC, AddressingMode::AbsoluteX, IGN, 4), Instruction::new(0xFD, AddressingMode::AbsoluteX, SBC, 4), Instruction::new(0xFE, AddressingMode::AbsoluteX, INC, 7), Instruction::new(0xFF, AddressingMode::AbsoluteX, ISC, 7),
  ];
}


impl CPU {

  pub fn decode(&mut self, op_code: u8) {
    let instruction = &INSTRUCTIONS[op_code as usize];

    let mode = &instruction.mode;

    // let instr_address = format!("{:X}", self.registers.pc - 1);
    // let instruction_name = instruction.name.to_string();

    // let register_a = format!("{:X}", self.registers.a);
    // let register_x = format!("{:X}", self.registers.x);
    // let register_y = format!("{:X}", self.registers.y);
    // let register_p = format!("{:X}", self.registers.p.bits());
    // let register_sp = format!("{:X}", self.registers.sp);

    // println!("{instr_address} {instruction_name} {mode}  A: {register_a} X: {register_x} Y: {register_y} P: {register_p} SP: {register_sp}");

    match instruction.name {
      ADC => self.adc(mode),
      ALR => self.alr(mode),
      ANC => self.anc(mode),
      AND => self.and(mode),
      ARR => self.arr(mode),
      ASL => self.asl(mode),
      AXS => self.axs(mode),
      BCC => self.branch(!self.registers.p.contains(CpuFlags::CARRY)),
      BCS => self.branch(self.registers.p.contains(CpuFlags::CARRY)),
      BEQ => self.branch(self.registers.p.contains(CpuFlags::ZERO)),
      BIT => self.bit(mode),
      BMI => self.branch(self.registers.p.contains(CpuFlags::NEGATIVE)),
      BNE => self.branch(!self.registers.p.contains(CpuFlags::ZERO)),
      BPL => self.branch(!self.registers.p.contains(CpuFlags::NEGATIVE)),
      BRK => self.brk(),
      BVC => self.branch(!self.registers.p.contains(CpuFlags::OVERFLOW)),
      BVS => self.branch(self.registers.p.contains(CpuFlags::OVERFLOW)),
      CLC => self.registers.p.remove(CpuFlags::CARRY),
      CLD => self.registers.p.remove(CpuFlags::DECIMAL_MODE),
      CLI => self.registers.p.remove(CpuFlags::INTERRUPT_DISABLE),
      CLV => self.registers.p.remove(CpuFlags::OVERFLOW),
      CMP => self.compare(mode, self.registers.a),
      CPX => self.compare(mode, self.registers.x),
      CPY => self.compare(mode, self.registers.y),
      DCP => self.dcp(mode),
      DEC => self.dec(mode),
      DEX => self.dex(),
      DEY => self.dey(),
      EOR => self.eor(mode),
      IGN => self.ign(mode),
      INC => self.inc(mode),
      INX => self.inx(),
      INY => self.iny(),
      ISC => self.isc(mode),
      JMP => self.jmp(mode),
      JSR => self.jsr(mode),
      LDA => self.lda(mode),
      LDX => self.ldx(mode),
      LDY => self.ldy(mode),
      LAX => self.lax(mode),
      LSR => self.lsr(mode),
      NOP => self.nop(),
      ORA => self.ora(mode),
      PHA => self.pha(),
      PHP => self.php(),
      PLA => self.pla(),
      PLP => self.plp(),
      RLA => self.rla(mode),
      ROL => self.rol(mode),
      ROR => self.ror(mode),
      RRA => self.rra(mode),
      RTI => self.rti(),
      RTS => self.rts(),
      SAX => self.sax(mode),
      SBC => self.sbc(mode),
      SEC => self.registers.p.insert(CpuFlags::CARRY),
      SED => self.registers.p.insert(CpuFlags::DECIMAL_MODE),
      SEI => self.registers.p.insert(CpuFlags::INTERRUPT_DISABLE),
      SKB => self.skb(mode),
      SLO => self.slo(mode),
      SRE => self.sre(mode),
      STA => self.store(mode, self.registers.a),
      STX => self.store(mode, self.registers.x),
      STY => self.store(mode, self.registers.y),
      TAX => self.tax(),
      TAY => self.tay(),
      TSX => self.tsx(),
      TXA => self.txa(),
      TXS => self.registers.sp = self.registers.x,
      TYA => self.tya(),
      _ => panic!("unknown op code received: {:X}", op_code),
    }

    self.cycle(instruction.cycles);
  }

  fn nop(&self) {
    // do nothing
  }

  fn ign(&mut self, mode: &AddressingMode) {
    let (address, page_cross) = self.get_operand_address(mode);

    self.mem_read(address);

    if page_cross {
      self.cycle(1);
    }
  }

  fn isc(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);
    let val = self.mem_read(address).wrapping_add(1);

    self.mem_write(address, val);

    self.subtract_carry(val);
  }

  fn slo(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);
    let mut val = self.mem_read(address);
    val = self.arithmetic_shift_left(val);

    self.mem_write(address, val);

    self.registers.a = self.registers.a | val;

    self.set_zero_and_negative_flags(self.registers.a);
  }

  fn sre(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);
    let mut val = self.mem_read(address);
    val = self.logical_shift_right(val);

    self.mem_write(address, val);

    self.registers.a ^= val;

    self.set_zero_and_negative_flags(self.registers.a);
  }

  fn rla(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);
    let mut val = self.mem_read(address);
    val = self.rotate_left(val);

    self.mem_write(address, val);

    self.registers.a &= val;

    self.set_zero_and_negative_flags(self.registers.a);
  }

  fn rra(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);
    let mut val = self.mem_read(address);
    val = self.rotate_right(val);

    self.mem_write(address, val);

    let carry = if self.registers.p.contains(CpuFlags::CARRY) { 1 } else { 0 };

    let (result, is_carry) = self.registers.a.overflowing_add(val);
    let (result2, is_carry2) = result.overflowing_add(carry);

    self.registers.p.set(CpuFlags::CARRY, is_carry || is_carry2);

    self.registers.p.set(CpuFlags::OVERFLOW, (val ^ result2) & (result2 ^ self.registers.a) & 0x80 != 0);

    self.registers.a = result2;

    self.set_zero_and_negative_flags(self.registers.a);
  }

  fn lax(&mut self, mode: &AddressingMode) {
    self.lda(mode);
    self.registers.x = self.registers.a;
  }

  fn dcp(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);
    let result = self.mem_read(address).wrapping_sub(1);

    self.mem_write(address, result);

    // CMP
    let cmp_result = self.registers.a.wrapping_sub(result);

    self.registers.p.set(CpuFlags::CARRY, self.registers.a >= result);

    self.set_zero_and_negative_flags(cmp_result);

  }

  fn alr(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);

    let val = self.mem_read(address);

    self.registers.a &= val;

    self.set_zero_and_negative_flags(self.registers.a);

    self.lsr_accumulator();
  }

  fn arr(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);

    let val = self.mem_read(address);

    self.registers.a &= val;

    self.set_zero_and_negative_flags(self.registers.a);

    self.ror_accumulator();

    let bit6 = (self.registers.a >> 6) & 0b1;
    let bit5 = (self.registers.a >> 5) & 0b1;

    self.registers.p.set(CpuFlags::CARRY, bit6 == 1);
    self.registers.p.set(CpuFlags::OVERFLOW, bit6 ^ bit5 == 1);

  }

  fn sax(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);

    let result = self.registers.a & self.registers.x;

    self.mem_write(address, result);
  }

  fn axs(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);
    let val = self.mem_read(address);

    let x_and_a = self.registers.x & self.registers.a;

    let result = x_and_a.wrapping_sub(val);

    self.registers.p.set(CpuFlags::CARRY, val <= x_and_a);

    self.registers.x = result;

    self.set_zero_and_negative_flags(result);
  }

  fn skb(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);

    self.mem_read(address);
  }

  fn tya(&mut self) {
    self.registers.a = self.registers.y;

    self.set_zero_and_negative_flags(self.registers.a);
  }

  fn txa(&mut self) {
    self.registers.a = self.registers.x;

    self.set_zero_and_negative_flags(self.registers.a);
  }

  fn tsx(&mut self) {
    self.registers.x = self.registers.sp;

    self.set_zero_and_negative_flags(self.registers.x);
  }

  fn store(&mut self, mode: &AddressingMode, value: u8) {
    let (address, _) = self.get_operand_address(mode);

    self.mem_write(address, value);
  }

  fn rts(&mut self) {
    self.registers.pc = self.pop_from_stack_u16() + 1;
  }

  fn rti(&mut self) {
    let byte = self.pop_from_stack();
    self.registers.p = CpuFlags::from_bits_truncate(byte);

    self.registers.p.remove(CpuFlags::BREAK);
    self.registers.p.insert(CpuFlags::BREAK2);

    self.registers.pc = self.pop_from_stack_u16();
  }

  fn rol(&mut self, mode: &AddressingMode) {
    if matches!(mode, AddressingMode::Accumulator) {
      return self.rol_accumulator();
    }

    let (address, _) = self.get_operand_address(mode);

    let mut val = self.mem_read(address);

    val = self.rotate_left(val);

    self.mem_write(address, val);

    self.set_zero_and_negative_flags(val);
  }

  fn rotate_left(&mut self, mut val: u8) -> u8 {
    let carry: u8 = if self.registers.p.contains(CpuFlags::CARRY) { 1 } else { 0 };

    self.registers.p.set(CpuFlags::CARRY, val >> 7 == 1);

    val = (val << 1) | carry;

    self.registers.p.set(CpuFlags::NEGATIVE, val >> 7 == 1);

    val
  }

  fn rol_accumulator(&mut self) {
    let carry: u8 = if self.registers.p.contains(CpuFlags::CARRY) { 1 } else { 0 };

    self.registers.p.set(CpuFlags::CARRY, self.registers.a >> 7 == 1);

    self.registers.a = (self.registers.a << 1) | carry;

    self.set_zero_and_negative_flags(self.registers.a);
  }

  fn ror(&mut self, mode: &AddressingMode) {
    if matches!(mode, AddressingMode::Accumulator) {
      return self.ror_accumulator();
    }

    let (address, _) = self.get_operand_address(mode);

    let mut val = self.mem_read(address);

    val = self.rotate_right(val);

    self.mem_write(address, val);

    self.set_zero_and_negative_flags(val);
  }

  fn rotate_right(&mut self, mut val: u8) -> u8 {
    let carry: u8 = if self.registers.p.contains(CpuFlags::CARRY) { 1 } else { 0 };

    self.registers.p.set(CpuFlags::CARRY, val & 0b1 == 1);

    val = (val >> 1) | (carry << 7);

    self.registers.p.set(CpuFlags::NEGATIVE, val >> 7 == 1);

    val
  }

  fn ror_accumulator(&mut self) {
    let carry: u8 = if self.registers.p.contains(CpuFlags::CARRY) { 1 } else { 0 };

    self.registers.p.set(CpuFlags::CARRY, self.registers.a & 0b1 == 1);

    self.registers.a = (self.registers.a >> 1) | (carry << 7);

    self.set_zero_and_negative_flags(self.registers.a);
  }

  fn pha(&mut self) {
    self.push_to_stack(self.registers.a);
  }

  fn php(&mut self) {

    let mut flags = self.registers.p.bits().clone();

    flags |= 0b1 << 4;
    flags |= 0b1 << 5;

    self.push_to_stack(flags);
  }

  fn pla(&mut self) {
    self.registers.a = self.pop_from_stack();
    self.set_zero_and_negative_flags(self.registers.a);
  }

  fn plp(&mut self) {
    let byte = self.pop_from_stack();
    self.registers.p = CpuFlags::from_bits_truncate(byte);

    self.registers.p.remove(CpuFlags::BREAK);
    self.registers.p.insert(CpuFlags::BREAK2);
  }

  fn compare(&mut self, mode: &AddressingMode, compare_to: u8) {
    let (address, page_cross) = self.get_operand_address(mode);

    let val = self.mem_read(address);

    let result = compare_to.wrapping_sub(val);

    self.registers.p.set(CpuFlags::CARRY, compare_to >= val);

    self.set_zero_and_negative_flags(result);

    if page_cross {
      self.cycle(1);
    }
  }

  fn inc(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);

    let result = self.mem_read(address).wrapping_add(1);

    self.set_zero_and_negative_flags(result);

    self.mem_write(address, result);
  }

  fn ora(&mut self, mode: &AddressingMode) {
    let (address, page_cross) = self.get_operand_address(mode);

    let val = self.mem_read(address);

    self.registers.a = self.registers.a | val;

    self.set_zero_and_negative_flags(self.registers.a);

    if page_cross {
      self.cycle(1);
    }
  }

  fn eor(&mut self, mode: &AddressingMode) {
    let (address, page_cross) = self.get_operand_address(mode);

    let val = self.mem_read(address);

    self.registers.a ^= val;

    self.set_zero_and_negative_flags(self.registers.a);

    if page_cross {
      self.cycle(1);
    }
  }

  fn inx(&mut self) {
    self.registers.x = self.registers.x.wrapping_add(1);

    self.set_zero_and_negative_flags(self.registers.x);
  }

  fn iny(&mut self) {
    self.registers.y = self.registers.y.wrapping_add(1);

    self.set_zero_and_negative_flags(self.registers.y);
  }

  fn bit(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);

    let val = self.mem_read(address);

    let result = val & self.registers.a;

    self.registers.p.set(CpuFlags::ZERO, result == 0);
    self.registers.p.set(CpuFlags::OVERFLOW, (val >> 6) & 0b1 == 1);
    self.registers.p.set(CpuFlags::NEGATIVE, val >> 7 == 1);
  }

  fn asl_accumulator(&mut self) {
    self.registers.a = self.arithmetic_shift_left(self.registers.a);
  }

  fn asl(&mut self, mode: &AddressingMode) {
    if matches!(mode, AddressingMode::Accumulator) {
      return self.asl_accumulator();
    }

    let (address, _) = self.get_operand_address(mode);

    let mut val = self.mem_read(address);
    val = self.arithmetic_shift_left(val);

    self.mem_write(address, val);
  }

  fn arithmetic_shift_left(&mut self, mut val: u8) -> u8 {
    self.registers.p.set(CpuFlags::CARRY, val >> 7 == 1);

    val = val << 1;

    self.set_zero_and_negative_flags(val);

    val
  }

  fn lsr(&mut self, mode: &AddressingMode) {
    if matches!(mode, AddressingMode::Accumulator) {
      return self.lsr_accumulator();
    }

    let (address, _) = self.get_operand_address(mode);

    let mut val = self.mem_read(address);
    val = self.logical_shift_right(val);

    self.mem_write(address, val);
  }

  fn logical_shift_right(&mut self, mut val: u8) -> u8 {
    self.registers.p.set(CpuFlags::CARRY, val & 0b1 == 1);

    val = val >> 1;

    self.set_zero_and_negative_flags(val);

    val
  }

  fn lsr_accumulator(&mut self) {
    self.registers.a = self.logical_shift_right(self.registers.a);
  }

  fn brk(&mut self) {
    self.registers.p.insert(CpuFlags::BREAK);
    self.registers.p.insert(CpuFlags::BREAK2);

    self.push_to_stack_u16(self.registers.pc+1);
    self.push_to_stack(self.registers.p.bits());

    let address = self.mem_read_u16(0xfffe);

    self.registers.pc = address;
  }

  fn anc(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);
    let val = self.mem_read(address);

    self.registers.a &= val;

    self.set_zero_and_negative_flags(self.registers.a);

    self.registers.p.set(CpuFlags::CARRY, self.registers.a >> 7 == 1)
  }

  fn and(&mut self, mode: &AddressingMode) {
    let (address, page_cross) = self.get_operand_address(mode);

    let val = self.mem_read(address);

    self.registers.a &= val;

    self.set_zero_and_negative_flags(self.registers.a);

    if page_cross {
      self.cycle(1);
    }
  }

  fn dec(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);

    let result = self.mem_read(address).wrapping_sub(1);

    self.mem_write(address, result);

    self.set_zero_and_negative_flags(result);
  }

  fn dex(&mut self) {
    self.registers.x = self.registers.x.wrapping_sub(1);

    self.set_zero_and_negative_flags(self.registers.x);
  }

  fn dey(&mut self) {
    self.registers.y = self.registers.y.wrapping_sub(1);

    self.set_zero_and_negative_flags(self.registers.y);
  }

  fn page_cross(base_address: u16, address: u16) -> bool {
    base_address & 0xFF00 != address & 0xFF00
  }

  fn get_operand_address(&mut self, mode: &AddressingMode) -> (u16, bool) {
    match mode {
      AddressingMode::Immediate => {
        let address = self.registers.pc;

        self.registers.pc += 1;

        (address, false)
      }
      AddressingMode::ZeroPage => {
        let address = self.mem_read(self.registers.pc) as u16;

        self.registers.pc += 1;

        (address, false)
      }
      AddressingMode::ZeroPageX => {
        let base_address = self.mem_read(self.registers.pc);

        let address = base_address.wrapping_add(self.registers.x) as u16;

        self.registers.pc += 1;

        (address, false)
      }
      AddressingMode::ZeroPageY => {
        let base_address = self.mem_read(self.registers.pc);

        let address = base_address.wrapping_add(self.registers.y) as u16;

        self.registers.pc += 1;

        (address, false)
      }
      AddressingMode::Absolute => {
        let address = self.mem_read_u16(self.registers.pc);

        self.registers.pc += 2;

        (address, false)
      }
      AddressingMode::AbsoluteX => {
        let tuple = self.get_absolute_offset_address(self.registers.x);

        self.registers.pc += 2;

        tuple
      }
      AddressingMode::AbsoluteY => {
        let tuple = self.get_absolute_offset_address(self.registers.y);

        self.registers.pc += 2;

        tuple
      }
      AddressingMode::Indirect => {
        let indirect_address = self.mem_read_u16(self.registers.pc);

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

        (address, false)
      }
      AddressingMode::IndirectX => {
        let base_address = self.mem_read(self.registers.pc);

        let actual_address = base_address.wrapping_add(self.registers.x);

        let low_byte = self.mem_read(actual_address as u16) as u16;
        let high_byte = self.mem_read(actual_address.wrapping_add(1) as u16) as u16;

        self.registers.pc += 1;

        let address = (high_byte << 8) | low_byte;

        (address, false)
      }
      AddressingMode::IndirectY => {
         let pointer = self.mem_read(self.registers.pc);

         let low_byte = self.mem_read(pointer as u16) as u16;
         let high_byte = self.mem_read(pointer.wrapping_add(1) as u16) as u16;

         let base = (high_byte << 8) | low_byte;
         let address = base.wrapping_add(self.registers.y as u16);

         self.registers.pc += 1;

         (address, Self::page_cross(base, address))
      }
      AddressingMode::NoneAddressing => panic!("mode is not supported"),
      AddressingMode::Accumulator => panic!("no address required for this mode")
    }
  }

  fn get_absolute_offset_address(&mut self, offset: u8) -> (u16, bool) {
    let base_address = self.mem_read_u16(self.registers.pc);

    let address = base_address.wrapping_add(offset as u16);

    (address, Self::page_cross(base_address, address))
  }

  fn lda(&mut self, mode: &AddressingMode) {
    let (address, page_cross) = self.get_operand_address(mode);

    let val = self.mem_read(address);

    self.registers.a = val;

    self.set_zero_and_negative_flags(val);

    if page_cross {
      self.cycle(1);
    }
  }

  fn ldx(&mut self, mode: &AddressingMode) {
    let (address, page_cross) = self.get_operand_address(mode);

    let val = self.mem_read(address);

    self.registers.x = val;

    self.set_zero_and_negative_flags(val);

    if page_cross {
      self.cycle(1);
    }
  }

  fn ldy(&mut self, mode: &AddressingMode) {
    let (address, page_cross) = self.get_operand_address(mode);

    let val = self.mem_read(address);

    self.registers.y = val;

    self.set_zero_and_negative_flags(val);

    if page_cross {
      self.cycle(1);
    }
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


    if (result >> 7) & 0b1 == 1 {
      self.registers.p.insert(CpuFlags::NEGATIVE);
    } else {
      self.registers.p.remove(CpuFlags::NEGATIVE);
    }
  }

  fn jmp(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);
    self.registers.pc = address;
  }

  fn jsr(&mut self, mode: &AddressingMode) {
    let (address, _) = self.get_operand_address(mode);

    self.push_to_stack_u16(self.registers.pc - 1);

    self.registers.pc = address;
  }

  fn adc(&mut self, mode: &AddressingMode) {
    let (address, page_cross) = self.get_operand_address(mode);

    let val = self.mem_read(address);

    let carry = if self.registers.p.contains(CpuFlags::CARRY) { 1 } else { 0 };

    let (result, is_carry) = self.registers.a.overflowing_add(val);
    let (result2, is_carry2) = result.overflowing_add(carry);

    self.registers.p.set(CpuFlags::CARRY, is_carry || is_carry2);

    self.registers.p.set(CpuFlags::OVERFLOW, (val ^ result2) & (result2 ^ self.registers.a) & 0x80 != 0);

    self.registers.a = result2;

    self.set_zero_and_negative_flags(self.registers.a);

    if page_cross {
      self.cycle(1);
    }
  }

  fn sbc(&mut self, mode: &AddressingMode) {
    let (address, page_cross) = self.get_operand_address(mode);

    let val = self.mem_read(address);

    self.subtract_carry(val);

    self.set_zero_and_negative_flags(self.registers.a);

    if page_cross {
      self.cycle(1);
    }

  }

  fn subtract_carry(&mut self, val: u8) {
    let carry_subtract: u8 = if !self.registers.p.contains(CpuFlags::CARRY) { 1 } else { 0 };

    let (result_without_carry, is_carry1) = self.registers.a.overflowing_sub(val);
    let (result_with_carry, is_carry2) = result_without_carry.overflowing_sub(carry_subtract);

    self.registers.p.set(CpuFlags::CARRY, !(is_carry1 || is_carry2));

    self.registers.p.set(CpuFlags::OVERFLOW, ((val ^ self.registers.a) & 0b10000000  == 0b10000000) && ((result_with_carry ^ self.registers.a) & 0b10000000 == 0b10000000));

    self.set_zero_and_negative_flags(result_with_carry);

    self.registers.a = result_with_carry;
  }

  fn branch(&mut self, condition: bool) {
    if condition {
      self.cycle(1);
      let val = self.mem_read(self.registers.pc) as i8;

      self.registers.pc += 1;

      let jump_address = self.registers.pc.wrapping_add_signed(val as i16);

      if self.registers.pc.wrapping_add(1) & 0xff00 != jump_address & 0xff00 {
        self.cycle(1);
      }

      self.registers.pc = jump_address;
    } else {
      self.registers.pc += 1;
    }
  }
}