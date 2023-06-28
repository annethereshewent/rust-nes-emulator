use std::collections::HashMap;

use nes_emulator::cpu::ppu::joypad::ButtonStatus;
use wasm_bindgen::prelude::*;

extern crate nes_emulator;

use nes_emulator::cpu::CPU;
use nes_emulator::cpu::ppu::CYCLES_PER_FRAME;
use nes_emulator::cartridge::Cartridge;

#[wasm_bindgen]
pub struct WasmEmulator {
  cpu: CPU,
  key_map: HashMap<ButtonEvent, ButtonStatus>
}

#[derive(PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub enum ButtonEvent {
  ButtonA,
  ButtonB,
  Select,
  Start,
  Up,
  Down,
  Left,
  Right
}

#[wasm_bindgen]
impl WasmEmulator {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    let mut key_map = HashMap::new();

    key_map.insert(ButtonEvent::ButtonA, ButtonStatus::BUTTON_A);
    key_map.insert(ButtonEvent::ButtonB, ButtonStatus::BUTTON_B);
    key_map.insert(ButtonEvent::Select, ButtonStatus::SELECT);
    key_map.insert(ButtonEvent::Start, ButtonStatus::START);
    key_map.insert(ButtonEvent::Up, ButtonStatus::UP);
    key_map.insert(ButtonEvent::Down, ButtonStatus::DOWN);
    key_map.insert(ButtonEvent::Left, ButtonStatus::LEFT);
    key_map.insert(ButtonEvent::Right, ButtonStatus::RIGHT);

    WasmEmulator {
      cpu: CPU::new(),
      key_map
    }
  }

  pub fn step_frame(&mut self) {
    let mut cycles = 0;

    while cycles < CYCLES_PER_FRAME {
      cycles += (self.cpu.tick()*3) as usize;
    }
  }

  pub fn get_picture_pointer(&self) -> *const u8 {
    self.cpu.ppu.picture.data.as_ptr()
  }

  pub fn load(&mut self, rom: &[u8]) {
    let cartridge = Cartridge::new(rom.to_vec());
    self.cpu.load_game(cartridge);
  }

  pub fn update_input(&mut self, button_event: ButtonEvent, is_pressed: bool) {
    if let Some(button) = self.key_map.get(&button_event) {
      self.cpu.ppu.joypad.set_button(*button, is_pressed);
    }
  }
}