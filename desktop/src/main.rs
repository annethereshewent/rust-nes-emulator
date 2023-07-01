extern crate nes_emulator;

use std::collections::HashMap;

use nes_emulator::cartridge::Cartridge;
use nes_emulator::cpu::CPU;

use nes_emulator::cpu::ppu::joypad::ButtonStatus;
use nes_emulator::cpu::ppu::{CYCLES_PER_FRAME, SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::audio::{AudioSpecDesired, AudioCallback};
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;

use std::{env, fs};


struct NesAudioCallback<'a> {
  volume: f32,
  cpu: &'a mut CPU
}

impl AudioCallback for NesAudioCallback<'_> {
  type Channel = f32;

  fn callback(&mut self, buf: &mut [Self::Channel]) {
    let mut index = 0;
    let mut apu = &mut self.cpu.apu;

    for b in buf.iter_mut() {
      *b = if index >= apu.buffer_index {
        apu.previous_value
      } else {
        apu.audio_samples[index]
      };

      apu.previous_value = *b;
      *b *= self.volume;
      index += 1;
    }

    apu.buffer_index = 0;
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Please specify a filename.");
  }

  let filepath = &args[1];

  let bytes: Vec<u8> = fs::read(filepath).unwrap();
  let mut cpu = CPU::new();

  cpu.load_game(Cartridge::new(bytes));

  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let audio_subsystem = sdl_context.audio().unwrap();

  let game_controller_subsystem = sdl_context.game_controller().unwrap();

  let spec = AudioSpecDesired {
    freq: Some(44100),
    channels: Some(1),
    samples: Some(4096)
  };

  let device = audio_subsystem.open_playback(
    None,
    &spec,
    |_| NesAudioCallback { volume: 0.5, cpu: &mut cpu }
  ).unwrap();

  device.resume();

  let available = game_controller_subsystem
      .num_joysticks()
      .map_err(|e| format!("can't enumerate joysticks: {}", e)).unwrap();

  let _controller = (0..available)
    .find_map(|id| {
      match game_controller_subsystem.open(id) {
        Ok(c) => {
          Some(c)
        }
        Err(_) => {
          None
        }
      }
    });

  let mut key_map = HashMap::new();

  key_map.insert(Keycode::W, ButtonStatus::UP);
  key_map.insert(Keycode::A, ButtonStatus::LEFT);
  key_map.insert(Keycode::S, ButtonStatus::DOWN);
  key_map.insert(Keycode::D, ButtonStatus::RIGHT);

  key_map.insert(Keycode::Space, ButtonStatus::BUTTON_A);
  key_map.insert(Keycode::K, ButtonStatus::BUTTON_A);

  key_map.insert(Keycode::LShift, ButtonStatus::BUTTON_B);
  key_map.insert(Keycode::J, ButtonStatus::BUTTON_B);

  key_map.insert(Keycode::Tab, ButtonStatus::SELECT);
  key_map.insert(Keycode::Return, ButtonStatus::START);


  let mut joypad_map = HashMap::new();

  joypad_map.insert(0, ButtonStatus::BUTTON_A);
  joypad_map.insert(2, ButtonStatus::BUTTON_B);

  joypad_map.insert(6, ButtonStatus::START);
  joypad_map.insert(4, ButtonStatus::SELECT);

  joypad_map.insert(11, ButtonStatus::UP);
  joypad_map.insert(12, ButtonStatus::DOWN);
  joypad_map.insert(13, ButtonStatus::LEFT);
  joypad_map.insert(14, ButtonStatus::RIGHT);

  let window = video_subsystem
    .window("NES Emulator", (SCREEN_WIDTH * 3) as u32, (SCREEN_HEIGHT * 3) as u32)
    .position_centered()
    .build()
    .unwrap();

  let mut canvas = window.into_canvas().present_vsync().build().unwrap();
  canvas.set_scale(3.0, 3.0).unwrap();

  let mut event_pump = sdl_context.event_pump().unwrap();

  let creator = canvas.texture_creator();
  let mut texture = creator
      .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
      .unwrap();

  loop {
    let mut cycles: usize = 0;
    while cycles < CYCLES_PER_FRAME {
      cycles += (cpu.tick() * 3) as usize;
    }

    cpu.ppu.cap_fps();


    // render the frame!
    texture.update(None, &cpu.ppu.picture.data, 256 * 3).unwrap();

    canvas.copy(&texture, None, None).unwrap();

    canvas.present();
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => std::process::exit(0),
        Event::KeyDown { keycode, .. }=> {
          if let Some(button) = key_map.get(&keycode.unwrap_or(Keycode::Return)){
            cpu.ppu.joypad.set_button(*button, true);
          }
        }
        Event::KeyUp { keycode, .. } => {
          if let Some(button) = key_map.get(&keycode.unwrap_or(Keycode::Return)){
            cpu.ppu.joypad.set_button(*button, false);
          }
        }
        Event::JoyButtonDown { button_idx, .. } => {
          if let Some(button) = joypad_map.get(&button_idx){
            cpu.ppu.joypad.set_button(*button, true);
          }
        }
        Event::JoyButtonUp { button_idx, .. } => {
          if let Some(button) = joypad_map.get(&button_idx){
            cpu.ppu.joypad.set_button(*button, false);
          }
        }
        _ => { /* do nothing */ }
      }
    }
  }
}
