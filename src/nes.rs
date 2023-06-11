pub mod cpu;

use cpu::CPU;
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
    let bytes: Vec<u8> = std::fs::read(filepath).unwrap();

    let mut cpu = CPU::new(bytes);

    for n in 1..100 {
      cpu.tick();
    }
  }
}