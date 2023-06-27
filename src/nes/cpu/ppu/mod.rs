pub mod registers;
pub mod picture;
pub mod joypad;

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use registers::control::ControlRegister;
use registers::mask::MaskRegister;
use registers::scroll::ScrollRegister;
use registers::status::StatusRegister;
use sdl2::EventPump;
use sdl2::controller::GameController;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use self::joypad::{ButtonStatus, Joypad};
use self::registers::address::AddressRegister;
use std::thread::sleep;

use picture::Picture;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;

use crate::nes::cartridge::Mirroring;

pub const SCANLINES_PER_FRAME: u16 = 262;
const CYCLES_PER_SCANLINE: u16 = 341;

pub const SCREEN_HEIGHT: u16 = 240;
pub const SCREEN_WIDTH: u16 = 256;

const MAX_FPS: u32 = 60;
const FPS_INTERVAL: u32 =  1000 / MAX_FPS;


// per https://github.com/kamiyaowl/rust-nes-emulator/blob/master/src/ppu_palette_table.rs
const PALETTE_TABLE: [(u8,u8,u8); 64] = [
  (84, 84, 84),
  (0, 30, 116 ),
  (8, 16, 144),
  (48, 0, 136),
  (68, 0, 100),
  (92, 0, 48),
  (84, 4, 0),
  (60, 24, 0),
  (32, 42, 0),
  (8, 58, 0),
  (0, 64, 0),
  (0, 60, 0),
  (0, 50, 60),
  (0, 0, 0),
  (0, 0, 0),
  (0, 0, 0),

  (152, 150, 152),
  (8, 76, 196 ),
  (48, 50, 236),
  (92, 30, 228),
  (136, 20, 176),
  (160, 20, 100),
  (152, 34, 32),
  (120, 60, 0),
  (84, 90, 0),
  (40, 114, 0),
  (8, 124, 0),
  (0, 118, 40),
  (0, 102, 120),
  (0, 0, 0),
  (0, 0, 0),
  (0, 0, 0),

  (236, 238, 236),
  (76, 154, 236 ),
  (120, 124, 236),
  (176, 98, 236),
  (228, 84, 236),
  (236, 88, 180),
  (236, 106, 100),
  (212, 136, 32),
  (160, 170, 0),
  (116, 196, 0),
  (76, 208, 32),
  (56, 204, 108),
  (56, 180, 204),
  (60, 60, 60),
  (0, 0, 0),
  (0, 0, 0),

  (236, 238, 236),
  (168, 204, 236),
  (188, 188, 236),
  (212, 178, 236),
  (236, 174, 236),
  (236, 174, 212),
  (236, 180, 176),
  (228, 196, 144),
  (204, 210, 120),
  (180, 222, 120),
  (168, 226, 144),
  (152, 226, 180),
  (160, 214, 228),
  (160, 162, 160),
  (0, 0, 0),
  (0, 0, 0),
];

pub struct PPU {
  ctrl: ControlRegister,
  mask: MaskRegister,
  scroll: ScrollRegister,
  status: StatusRegister,
  ppu_addr: AddressRegister,
  pub palette_table: [u8; 32],
  pub chr_rom: Vec<u8>,
  pub vram: [u8; 2048],
  pub oam_data: [u8; 256],
  pub oam_address: u8,
  pub mirroring: Mirroring,
  internal_data: u8,
  cycles: u16,
  current_scanline: u16,
  pub nmi_triggered: bool,
  canvas: Canvas<Window>,
  picture: Picture,
  event_pump: EventPump,
  _controller: Option<GameController>,
  pub joypad: Joypad,
  joypad_map: HashMap<u8, ButtonStatus>,
  key_map: HashMap<Keycode, ButtonStatus>,
  background_pixels_drawn: Vec<bool>,
  previous_time: u128
}

impl PPU {
  pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let game_controller_subsystem = sdl_context.game_controller().unwrap();

    let available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e)).unwrap();

    let controller = (0..available)
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

    let event_pump = sdl_context.event_pump().unwrap();

    PPU {
      ctrl: ControlRegister::from_bits_truncate(0b00000000),
      mask: MaskRegister::from_bits_truncate(0b00000000),
      scroll: ScrollRegister::new(),
      status: StatusRegister::from_bits_truncate(0b00000000),
      ppu_addr: AddressRegister::new(),
      chr_rom,
      oam_data: [0; 256],
      oam_address: 0,
      vram: [0; 2048],
      mirroring,
      internal_data: 0,
      palette_table: [0; 32],
      cycles: 0,
      current_scanline: 0,
      nmi_triggered: false,
      canvas,
      picture: Picture::new(),
      event_pump,
      _controller: controller,
      joypad_map,
      key_map,
      joypad: Joypad::new(),
      background_pixels_drawn: Vec::new(),
      previous_time: 0
    }
  }

  pub fn tick(&mut self, cycles: u16) {
    self.cycles += cycles;

    if self.cycles >= CYCLES_PER_SCANLINE {
      if self.sprite_zero_hit(self.cycles) {
        self.status.insert(StatusRegister::SPRITE_ZERO_HIT);
      }

      self.cycles -= CYCLES_PER_SCANLINE;
      if self.current_scanline < SCREEN_HEIGHT {
        self.draw_line();
      }

      self.current_scanline += 1;

      if self.current_scanline == SCREEN_HEIGHT+1 {
        // trigger NMI interrupt
        self.status.remove(StatusRegister::SPRITE_ZERO_HIT);
        self.status.insert(StatusRegister::VBLANK_STARTED);
        if self.ctrl.generate_nmi_interrupt() {
          self.nmi_triggered = true;
        }
      }

      if self.current_scanline >= SCANLINES_PER_FRAME {
        self.cap_fps();

        self.render();
        self.current_scanline = 0;
        self.nmi_triggered = false;
        self.status.remove(StatusRegister::VBLANK_STARTED);
        self.status.remove(StatusRegister::SPRITE_ZERO_HIT);
      }
    }
  }

  fn cap_fps(&mut self) {
    let current_time = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("an error occurred")
      .as_millis();

    if self.previous_time != 0 {
      let diff = current_time - self.previous_time;
      if diff < FPS_INTERVAL as u128 {
        // sleep for the missing time
        sleep(Duration::from_millis((FPS_INTERVAL - diff as u32) as u64));
      }
    }

    self.previous_time = current_time;
  }
  fn sprite_zero_hit(&self, cycles: u16) -> bool {
    let y = self.oam_data[0];
    let x = self.oam_data[3];

    y as u16 == self.current_scanline && x as u16 <= cycles && self.mask.contains(MaskRegister::SHOW_SPRITES)
  }

  // see https://www.nesdev.org/wiki/Mirroring
  fn mirror_vram_index(&self, address: u16) -> u16 {
    let mirrored_address = address & 0b10111111111111; // mirror down address to range 0x2000 to 0x2eff, where nametables exist
    let vram_index = mirrored_address - 0x2000;
    let name_table_index = vram_index / 0x400; // this should give us a value between 0-3 which points to what nametable is being referred to

    match (&self.mirroring, name_table_index) {
      (Mirroring::Horizontal, 1) => vram_index - 0x400, // first kb of memory
      (Mirroring::Horizontal, 2) => vram_index - 0x400, // 2nd kb of memory
      (Mirroring::Horizontal, 3) => vram_index - 0x800, // 2nd kb of memory
      (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index- 0x800, // 2 is in first kb of memory 3 is in 2nd (ie: if vram index is 0xc00 [index 2], subtracting 0x800 would put it at 0x400, start of 2nd kb of ram)
      _ => vram_index // either it's four screen which has no mirroring or it's nametable 0 or another nametable that doesn't need the offset
    }
  }

  pub fn read_status_register(&mut self) -> u8 {
    let data = self.status.bits();

    self.scroll.latch = false;
    self.ppu_addr.latch = false;
    self.status.set(StatusRegister::VBLANK_STARTED, false);

    data
  }

  fn draw_line(&mut self) {
    self.background_pixels_drawn = Vec::new();
    self.draw_background();
    self.draw_sprites();
  }

  fn draw_sprites(&mut self) {
    let y = self.current_scanline;

    for i in (0..self.oam_data.len()).step_by(4) {
      let tile_y = self.oam_data[i];
      let tile_number = self.oam_data[i+1];
      let attributes = self.oam_data[i+2];
      let tile_x = self.oam_data[i+3];

      let y_flip = (attributes >> 7) & 0b1 == 1;
      let x_flip = (attributes >> 6) & 0b1 == 1;

      let sprite_behind_background = (attributes >> 5) & 0b1 == 1;

      let mut y_pos_in_tile: i16 = (y as i16) - (tile_y as i16);

      if y_flip {
        y_pos_in_tile = self.ctrl.sprite_size() as i16 - 1 - y_pos_in_tile;
      }

      if y_pos_in_tile >= 0 && (y_pos_in_tile as u16) < self.ctrl.sprite_size() as u16 {
        let palette_index = attributes & 0b11;

        let sprite_palettes = self.get_sprite_palette(palette_index);

        let bank = self.ctrl.sprite_pattern_table_address();

        let tile_index = bank + tile_number as u16 * 16;

        let lower_byte = self.chr_rom[(tile_index + y_pos_in_tile as u16) as usize];
        let upper_byte = self.chr_rom[(tile_index + y_pos_in_tile as u16 + 8) as usize];

        for x in 0..8 {
          let bit_pos = if x_flip {
            x
          } else {
            7 - x
          };

          let color_index = ((lower_byte >> bit_pos) & 0b1) + (((upper_byte >> bit_pos) & 0b1) << 1);

          let rgb = match color_index {
            0 => continue,
            _ => PALETTE_TABLE[sprite_palettes[color_index as usize] as usize]
          };

          let x_pos = (tile_x as usize + x) as usize;

          if x_pos >= SCREEN_WIDTH as usize {
            continue;
          }

          let is_pixel_visible = !(sprite_behind_background && self.background_pixels_drawn[x_pos]);

          if is_pixel_visible && x_pos > 0 {
            self.picture.set_pixel(x_pos, y as usize, rgb);
          }
        }

      }
    }
  }

  fn draw_background(&mut self) {
    let nametable_base = self.ctrl.base_table_address();

    let second_nametable_base = match (nametable_base, &self.mirroring) {
      (0x2000, Mirroring::Vertical) | (0x2800, Mirroring::Vertical) => 0x2400,
      (0x2400, Mirroring::Vertical) | (0x2c00, Mirroring::Vertical) | (0x2800, Mirroring::Horizontal) | (0x2c00, Mirroring::Horizontal) => 0x2000,
      (0x2400, Mirroring::Horizontal) | (0x2000, Mirroring::Horizontal)  => 0x2800,
      _ => todo!("not yet implemented")
    };

    let chr_rom_bank = if self.ctrl.sprite_size() == 8 { self.ctrl.background_pattern_table_addr() } else { 0 };

    let y = self.current_scanline;

    // let mut scrolled_y = y;

    for x in 0..SCREEN_WIDTH {
      let mut scrolled_x = x + self.scroll.x as u16;
      let mut scrolled_y = self.scroll.y as u16 + y;

      let current_nametable = if matches!(&self.mirroring, Mirroring::Vertical) {
        if scrolled_x < SCREEN_WIDTH {
          nametable_base
        } else {
          scrolled_x %= SCREEN_WIDTH;
          second_nametable_base
        }
      } else if matches!(&self.mirroring, Mirroring::Horizontal) {
        if scrolled_y < SCREEN_HEIGHT {
          nametable_base
        } else {
          scrolled_y %= SCREEN_HEIGHT;
          second_nametable_base
        }
      } else {
        todo!("four screen not implemented");
      };

      let tile_pos = (scrolled_x / 8) + (scrolled_y / 8) * 32;

      let tile_number = self.vram[self.mirror_vram_index(current_nametable + tile_pos) as usize];

      let tile_index = chr_rom_bank + (tile_number as u16 * 16);

      let x_pos_in_tile = scrolled_x % 8;
      let y_pos_in_tile = scrolled_y % 8;

      let lower_byte = self.chr_rom[(tile_index + y_pos_in_tile) as usize];
      let upper_byte = self.chr_rom[(tile_index + y_pos_in_tile + 8) as usize];

      let bit_pos = 7 - x_pos_in_tile;

      let color_index = ((lower_byte >> bit_pos) & 0b1) + (((upper_byte >> bit_pos) & 0b1) << 1);

      let tile_column = scrolled_x / 8;
      let tile_row = scrolled_y / 8;

      let bg_palette = self.get_bg_palette(current_nametable as usize, tile_column as usize, tile_row as usize);


      self.background_pixels_drawn.push(color_index != 0);

      // finally render the pixel!
      let rgb = PALETTE_TABLE[bg_palette[color_index as usize] as usize];
      self.picture.set_pixel(x as usize, y as usize, rgb);

    }
  }

  fn get_sprite_palette(&self, palette_index: u8) -> [u8; 4] {
    // there are 0x11 (or 17) indexes for the background palettes
    let start = 0x11 + (palette_index * 4) as usize;

    [
      0,
      self.palette_table[start],
      self.palette_table[start+1],
      self.palette_table[start+2]
    ]
  }

  fn get_bg_palette(&self, nametable_base: usize, tile_column: usize, tile_row: usize) -> [u8; 4] {
    // 1 byte in attribute table controls the palette for 4 neighboring meta-tiles, where a meta-tile is 2x2 tiles
    // thus, 1 byte controls 4x4 tiles or 32x32 pixels total.
    // so in order to get the index, divide the tile column position by 4, and the tile row position by 4 and multiply by 8
    // (8 * 32 = 256, the screen width)
    let attribute_table_index = (tile_row / 4) * 8 + (tile_column / 4);

    let attr_byte = self.vram[self.mirror_vram_index((nametable_base + 0x3c0 + attribute_table_index) as u16) as usize];

    let x_meta_tile_pos = (tile_column % 4) / 2;
    let y_meta_tile_pos = (tile_row % 4) / 2;

    // once you have the x,y coordinates within the 4 neighboring meta tiles, you can determine what two bits to use for the palette
    // for instance, 0,0 is the top left meta-tile, 1,0 is the top right, and so on.
    // the first two bits determine the first meta tile, 2nd 2 bits determine the 2nd, 3rd determine the 3rd, last two bits determine 4th tile
    let palette_index = match (x_meta_tile_pos, y_meta_tile_pos) {
      (0,0) => attr_byte & 0b11,
      (1,0) => (attr_byte >> 2) & 0b11,
      (0,1) => (attr_byte >> 4) & 0b11,
      (1,1) => (attr_byte >> 6) & 0b11,
      _ => panic!("should not get here")
    };

    // despite there being 3 colors per palette, after each palette an index is skipped, hence the * 4
    // ie: palette 0 starts at 0x01 and ends at 0x03, but palette 1 doesn't start until 0x05
    let palette_start: usize = 1 + (palette_index as usize * 4);

    [self.palette_table[0], self.palette_table[palette_start], self.palette_table[palette_start+1], self.palette_table[palette_start+2]]
  }

  fn render(&mut self) {
    let creator = self.canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    texture.update(None, &self.picture.data, 256 * 3).unwrap();

    self.canvas.copy(&texture, None, None).unwrap();

    self.canvas.present();

    for event in self.event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => std::process::exit(0),
        Event::KeyDown { keycode, .. }=> {
          if let Some(button) = self.key_map.get(&keycode.unwrap_or(Keycode::Return)){
            self.joypad.set_button(*button, true);
          }
        }
        Event::KeyUp { keycode, .. } => {
          if let Some(button) = self.key_map.get(&keycode.unwrap_or(Keycode::Return)){
            self.joypad.set_button(*button, false);
          }
        }
        Event::JoyButtonDown { button_idx, .. } => {
          if let Some(button) = self.joypad_map.get(&button_idx){
            self.joypad.set_button(*button, true);
          }
        }
        Event::JoyButtonUp { button_idx, .. } => {
          if let Some(button) = self.joypad_map.get(&button_idx){
            self.joypad.set_button(*button, false);
          }
        }
        _ => { /* do nothing */ }
      }
    }
  }

  pub fn read_oam_data(&self) -> u8 {
    self.oam_data[self.oam_address as usize]
  }

  pub fn write_to_control(&mut self, value: u8) {
    let tmp = self.ctrl.generate_nmi_interrupt();
    self.ctrl = ControlRegister::from_bits_truncate(value);
    if !tmp && self.status.contains(StatusRegister::VBLANK_STARTED) && self.ctrl.generate_nmi_interrupt() {
      self.nmi_triggered = true;
    }
  }

  pub fn write_to_mask(&mut self, value: u8) {
    self.mask = MaskRegister::from_bits_truncate(value);
  }

  pub fn write_to_oam_address(&mut self, value: u8) {
    self.oam_address = value;
  }

  pub fn write_to_oam_data(&mut self, value: u8) {
    self.oam_data[self.oam_address as usize] = value;

    self.oam_address = self.oam_address.wrapping_add(1);
  }

  pub fn write_to_scroll(&mut self, value: u8) {
    self.scroll.set(value);
  }

  pub fn write_to_ppu_address(&mut self, value: u8) {
    self.ppu_addr.update(value);
  }

  pub fn write_to_data(&mut self, value: u8) {
    let address = self.ppu_addr.get();

    match address {
      0x0000 ..= 0x1fff => println!("attempting to write to chr rom, {:X}", address),
      0x2000 ..=0x2fff => self.vram[self.mirror_vram_index(address) as usize] = value,
      0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
        let address_mirror = address - 0x10;
        self.palette_table[(address_mirror - 0x3f00) as usize] = value;
      }
      0x3f00 ..=0x3fff => self.palette_table[(address - 0x3f00) as usize] = value,
      _ => panic!("shouldn't get here")
    }

    self.ppu_addr.increment(self.ctrl.vram_address_increment());
  }

  pub fn read_data(&mut self) -> u8 {
    let address = self.ppu_addr.get();

    self.ppu_addr.increment(self.ctrl.vram_address_increment());

    match address {
      0x0000 ..= 0x1fff => {
        let result = self.internal_data;

        self.internal_data = self.chr_rom[address as usize];

        result
      },
      0x2000 ..= 0x2fff => {
        let result = self.internal_data;

        self.internal_data = self.vram[self.mirror_vram_index(address) as usize];

        result
      }
      0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
        let address_mirror = address - 0x10;
        self.palette_table[(address_mirror - 0x3f00) as usize]
      }

      0x3f00..=0x3fff =>
      {
        self.palette_table[(address - 0x3f00) as usize]
      }
      _ => panic!("shouldn't get here")
    }
  }
}