use wasm_bindgen::prelude::*;

extern crate nes_emulator;

use nes_emulator::cpu::CPU;
use nes_emulator::cpu::ppu::CYCLES_PER_FRAME;
use nes_emulator::cartridge::Cartridge;

#[wasm_bindgen]
pub struct WasmEmulator {
  cpu: CPU
}

#[wasm_bindgen]
impl WasmEmulator {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    WasmEmulator {
      cpu: CPU::new()
    }
  }

  #[wasm_bindgen]
  pub fn step_frame(&mut self) {
    let mut cycles = 0;

    while cycles < CYCLES_PER_FRAME {
      cycles += self.cpu.tick() as usize;
    }
  }

  #[wasm_bindgen]
  pub fn get_picture_pointer(&self) -> *const u8 {
    self.cpu.ppu.picture.data.as_ptr()
  }

  #[wasm_bindgen]
  pub fn load(&mut self, rom: &[u8]) {
    let cartridge = Cartridge::new(rom.to_vec());
    self.cpu.load_game(cartridge);
  }
}