pub mod cpu;
pub mod cartridge;

use cpu::CPU;
use cartridge::Cartridge;
use std::fs;

pub struct NES {
  is_running: bool
}

impl NES {
  pub fn new() -> Self {
    NES {
      is_running: true
    }
  }

  pub fn run(&mut self, filepath: &String) {
    println!("the filepath is {}", filepath);
    let bytes: Vec<u8> = fs::read(filepath).unwrap();

    let cartridge = Cartridge::new(bytes);

    let mut cpu = CPU::new(cartridge);

    for _n in 1..100 {
      if self.is_running {
        cpu.tick();
      } else {
        break;
      }
    }
  }
}