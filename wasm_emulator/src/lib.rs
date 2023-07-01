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
  key_map: HashMap<ButtonEvent, ButtonStatus>,
  read_index: u16
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
      key_map,
      read_index: 0
    }
  }

  pub fn set_buffer_index(&mut self, index: usize) {
    self.cpu.apu.buffer_index = index;
  }

  pub fn get_audio_sample_pointer(&self) -> *const f32 {
    self.cpu.apu.audio_samples.as_ptr()
  }

  pub fn get_buffer_index(&self) -> usize {
    self.cpu.apu.buffer_index
  }

  pub fn get_read_index(&self) -> u16 {
    self.read_index
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