pub mod cpu;
pub mod cartridge;
pub mod wasm_emulator;

use cpu::CPU;
use cartridge::Cartridge;
use std::fs;

use wasm_emulator::WasmEmulator;

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
    let bytes: Vec<u8> = fs::read(filepath).unwrap();

    let cartridge = Cartridge::new(bytes);

    let mut cpu = CPU::new();
    cpu.load_game(cartridge);

    loop {
      if self.is_running {
        cpu.tick();
      } else {
        break;
      }
    }
  }
}