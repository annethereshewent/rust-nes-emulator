
pub mod OpCodes {
  use crate::nes::CPU;

  pub fn decode(cpu: &mut CPU, op_code: u8) {
    println!("found op code with code {}", format!("{:X}", op_code));
    match op_code {
      0xaa => {
        cpu.registers.x = cpu.registers.a;

        if cpu.registers.x == 0 {
          cpu.registers.p = cpu.registers.p | (0b1 << 1);
        } else {
          cpu.registers.p = cpu.registers.p & !(0b1 << 1)
        }
      }
      _ => todo()
    }
  }

  fn todo() {

  }
}