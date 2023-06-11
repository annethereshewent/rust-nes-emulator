use crate::nes::CPU;

impl CPU {
  pub fn decode(&mut self, op_code: u8) {
    match op_code {
      0x00 => return,
      0xa9 => self.lda(),
      0xaa => self.tax(),
      0xe8 => self.inx(),
      _ => self.todo()
    }
  }

  fn inx(&mut self) {
    self.registers.x += 1;

    self.set_zero_and_negative_flags(self.registers.x);
  }

  fn lda(&mut self) {
    let val = self.program[self.registers.pc as usize];

    self.registers.pc += 1;

    self.registers.a = val;

    self.set_zero_and_negative_flags(val);
  }

  fn tax(&mut self) {
    self.registers.x = self.registers.a;

    self.set_zero_and_negative_flags(self.registers.x);
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