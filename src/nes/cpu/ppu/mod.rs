pub mod registers;
pub mod picture;

use registers::control::ControlRegister;
use registers::mask::MaskRegister;
use registers::scroll::ScrollRegister;
use registers::status::StatusRegister;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use self::registers::address::AddressRegister;

use picture::Picture;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;

use crate::nes::cartridge::Mirroring;

const SCANLINES_PER_FRAME: u16 = 262;
const CYCLES_PER_SCANLINE: u16 = 341;

pub const SCREEN_HEIGHT: u16 = 240;
pub const SCREEN_WIDTH: u16 = 256;

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
  event_pump: EventPump
}

impl PPU {
  pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

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
      event_pump
    }
  }

  pub fn tick(&mut self, cycles: u16) {
    self.cycles += cycles;

    if self.cycles >= CYCLES_PER_SCANLINE {
      self.cycles -= CYCLES_PER_SCANLINE;
      if self.current_scanline >= SCREEN_HEIGHT {
        // trigger NMI interrupt
        self.status.insert(StatusRegister::VBLANK_STARTED);
        if self.ctrl.generate_nmi_interrupt() {
          self.nmi_triggered = true;
        }
      } else {
        self.draw_line();
      }

      self.current_scanline += 1;

      if self.current_scanline >= SCANLINES_PER_FRAME {
        self.render();
        self.current_scanline = 0;
        self.status.remove(StatusRegister::VBLANK_STARTED);
      }
    }
  }

  // see https://www.nesdev.org/wiki/Mirroring
  // also https://bugzmanov.github.io/nes_ebook/chapter_6_1.html
  fn mirror_vram_index(&self, address: u16) -> u16 {
    let mirrored_address = address & 0b10111111111111; // mirror down address to range 0x2000 to 0x2eff, where nametables exist
    let vram_index = mirrored_address - 0x2000;
    let name_table_index = vram_index / 0x400; // this should give us a value between 0-3 which points to what nametable is being referred to

    match (&self.mirroring, name_table_index) {
      (Mirroring::Horizontal, 1) => vram_index - 0x400, // first kb of memory
      (Mirroring::Horizontal, 2) => vram_index - 0x400, // 2nd kb of memory
      (Mirroring::Horizontal, 3) => vram_index - 0x800, // 2nd kb of memory
      (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index- 0x800, // 2 is in first kb of memory 3 is in 2nd (ie: if vram index is 0xc00 [index 2], subtracting 0x800 would put it at 0x400, start of 2nd kb of ram)
      _ => vram_index // either it's four screen which has no mirroring or it's screen 0 or another screen that doesn't need the offset
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
    self.draw_background();
    self.draw_sprites();
  }

  fn draw_sprites(&mut self) {
    let y = self.current_scanline;

    for i in (0..self.oam_data.len()).step_by(4) {
      let tile_x = self.oam_data[i+3];
      let tile_y = self.oam_data[i];

      let tile_number = self.oam_data[i+1];
      let attributes = self.oam_data[i+2];

      let y_flip = (attributes >> 7) & 0b1 == 1;
      let x_flip = (attributes >> 6) & 0b1 == 1;

      let mut y_intersection: i16 = (y as i16) - (tile_y as i16);

      if y_flip {
        y_intersection = self.ctrl.sprite_size() as i16 - 1 - y_intersection as i16;
      }

      if y_intersection >= 0 && (y_intersection as u16) < self.ctrl.sprite_size() as u16 {
        let palette_index = attributes & 0b11;

        let sprite_palettes = self.get_sprite_palette(palette_index);

        let bank = self.ctrl.sprite_pattern_table_address();

        let tile_index = bank + tile_number as u16 * 16;

        let lower_byte = self.chr_rom[(tile_index + y_intersection as u16) as usize];
        let upper_byte = self.chr_rom[(tile_index + y_intersection as u16 + 8) as usize];

        for x in 0..8 {
          let x_shift = if x_flip {
            x
          } else {
            7 - x
          };

          let color_index = ((lower_byte >> x_shift) & 0b1) + (((upper_byte >> x_shift) & 0b1) << 1);

          let rgb = match color_index {
            0 => continue,
            1 => PALETTE_TABLE[sprite_palettes[1] as usize],
            2 => PALETTE_TABLE[sprite_palettes[2] as usize],
            3 => PALETTE_TABLE[sprite_palettes[3] as usize],
            _ => panic!("cant happen")
          };

          let x_pos = (tile_x + x) as usize;

          self.picture.set_pixel(x_pos, y as usize, rgb);
        }

      }
    }
  }

  fn draw_background(&mut self) {
    let nametable_base = self.ctrl.base_table_address();
    let chr_rom_bank = self.ctrl.background_pattern_table_addr();

    let y = self.current_scanline;

    for x in 0..SCREEN_WIDTH {
      let tile_pos = (x / 8) + (y / 8) * 32;
      let tile_number = self.vram[self.mirror_vram_index(nametable_base + tile_pos) as usize];
      let tile_index = chr_rom_bank + (tile_number as u16 * 16);

      let x_pos_in_tile = x % 8;
      let y_pos_in_tile = y % 8;

      let lower_byte = self.chr_rom[(tile_index + y_pos_in_tile) as usize];
      let upper_byte = self.chr_rom[(tile_index + y_pos_in_tile + 8) as usize];

      let x_shift = 7 - x_pos_in_tile;

      let color_index = ((lower_byte >> x_shift) & 0b1) + (((upper_byte >> x_shift) & 0b1) << 1);

      let tile_column = x / 8;
      let tile_row = y / 8;

      let bg_palette = self.get_bg_palette(nametable_base as usize, tile_column as usize, tile_row as usize);

      // finally render the pixel!
      let rgb = match color_index {
        0 => PALETTE_TABLE[self.palette_table[0] as usize],
        1 => PALETTE_TABLE[bg_palette[1] as usize],
        2 => PALETTE_TABLE[bg_palette[2] as usize],
        3 => PALETTE_TABLE[bg_palette[3]as usize],
        _ => panic!("shouldn't get here")
      };
      self.picture.set_pixel(x as usize, y as usize, rgb);
    }
  }

  fn get_sprite_palette(&self, palette_index: u8) -> [u8; 4] {
    let start = 0x11 + (palette_index * 4) as usize;

    [
      0,
      self.palette_table[start],
      self.palette_table[start+1],
      self.palette_table[start+2]
    ]
  }

  // see https://bugzmanov.github.io/nes_ebook/chapter_6_4.html on rendering colors and meta-tiles
  fn get_bg_palette(&self, nametable_base: usize, tile_column: usize, tile_row: usize) -> [u8; 4] {
    let attribute_table_index = (tile_row / 4) * 8 + (tile_column / 4);

    let attr_byte = self.vram[self.mirror_vram_index((nametable_base + 0x3c0 + attribute_table_index) as u16) as usize];

    let x_meta_tile_pos = (tile_column % 4) / 2;
    let y_meta_tile_pos = (tile_row % 4) / 2;

    let palette_index = match (x_meta_tile_pos, y_meta_tile_pos) {
      (0,0) => attr_byte & 0b11,
      (0,1) => (attr_byte >> 4) & 0b11,
      (1,0) => (attr_byte >> 2) & 0b11,
      (1,1) => (attr_byte >> 6) & 0b11,
      _ => panic!("should not get here")
    };

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

    self.ppu_addr.increment(self.ctrl.vram_address_increment());

    match address {
      0x0000 ..= 0x1fff => panic!("attempt to write to chr rom"),
      0x2000 ..=0x2fff => self.vram[self.mirror_vram_index(address) as usize] = value,
      0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
        let address_mirror = address - 0x10;
        self.palette_table[(address_mirror - 0x3f00) as usize] = value;
      }
      0x3f00 ..=0x3fff => self.palette_table[(address - 0x3f00) as usize] = value,
      _ => panic!("shouldn't get here")
    }
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