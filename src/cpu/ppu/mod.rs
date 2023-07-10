pub mod registers;
pub mod picture;
pub mod joypad;

use std::thread::sleep;
use std::time::{Duration, UNIX_EPOCH, SystemTime};

use registers::control::ControlRegister;
use registers::mask::MaskRegister;
use registers::scroll::ScrollRegister;
use registers::status::StatusRegister;
use self::joypad::Joypad;

use picture::Picture;

use crate::cartridge::Mirroring;
use crate::mapper::{Mapper, Empty, MapperActions};

pub const SCANLINES_PER_FRAME: u16 = 262;
const CYCLES_PER_SCANLINE: u16 = 341;

pub const CYCLES_PER_FRAME: usize = CYCLES_PER_SCANLINE as usize * SCANLINES_PER_FRAME as usize;

pub const SCREEN_HEIGHT: u16 = 240;
pub const SCREEN_WIDTH: u16 = 256;

const MAX_FPS: u32 = 60;
pub const FPS_INTERVAL: u32 =  1000 / MAX_FPS;

const PRERENDER_SCANLINE: u16 = 261;

const OAM_FETCH_START: u16 = 257;


// per https://github.com/kamiyaowl/rust-nes-emulator/blob/master/src/ppu_palette_table.rs
// const PALETTE_TABLE: [(u8,u8,u8); 64] = [
//   (84, 84, 84),
//   (0, 30, 116 ),
//   (8, 16, 144),
//   (48, 0, 136),
//   (68, 0, 100),
//   (92, 0, 48),
//   (84, 4, 0),
//   (60, 24, 0),
//   (32, 42, 0),
//   (8, 58, 0),
//   (0, 64, 0),
//   (0, 60, 0),
//   (0, 50, 60),
//   (0, 0, 0),
//   (0, 0, 0),
//   (0, 0, 0),

//   (152, 150, 152),
//   (8, 76, 196 ),
//   (48, 50, 236),
//   (92, 30, 228),
//   (136, 20, 176),
//   (160, 20, 100),
//   (152, 34, 32),
//   (120, 60, 0),
//   (84, 90, 0),
//   (40, 114, 0),
//   (8, 124, 0),
//   (0, 118, 40),
//   (0, 102, 120),
//   (0, 0, 0),
//   (0, 0, 0),
//   (0, 0, 0),

//   (236, 238, 236),
//   (76, 154, 236 ),
//   (120, 124, 236),
//   (176, 98, 236),
//   (228, 84, 236),
//   (236, 88, 180),
//   (236, 106, 100),
//   (212, 136, 32),
//   (160, 170, 0),
//   (116, 196, 0),
//   (76, 208, 32),
//   (56, 204, 108),
//   (56, 180, 204),
//   (60, 60, 60),
//   (0, 0, 0),
//   (0, 0, 0),

//   (236, 238, 236),
//   (168, 204, 236),
//   (188, 188, 236),
//   (212, 178, 236),
//   (236, 174, 236),
//   (236, 174, 212),
//   (236, 180, 176),
//   (228, 196, 144),
//   (204, 210, 120),
//   (180, 222, 120),
//   (168, 226, 144),
//   (152, 226, 180),
//   (160, 214, 228),
//   (160, 162, 160),
//   (0, 0, 0),
//   (0, 0, 0),
// ];

// per https://bugzmanov.github.io/nes_ebook/chapter_6_3.html
const PALETTE_TABLE: [(u8, u8, u8); 64] = [
  (0x80, 0x80, 0x80), (0x00, 0x3D, 0xA6), (0x00, 0x12, 0xB0), (0x44, 0x00, 0x96), (0xA1, 0x00, 0x5E),
  (0xC7, 0x00, 0x28), (0xBA, 0x06, 0x00), (0x8C, 0x17, 0x00), (0x5C, 0x2F, 0x00), (0x10, 0x45, 0x00),
  (0x05, 0x4A, 0x00), (0x00, 0x47, 0x2E), (0x00, 0x41, 0x66), (0x00, 0x00, 0x00), (0x05, 0x05, 0x05),
  (0x05, 0x05, 0x05), (0xC7, 0xC7, 0xC7), (0x00, 0x77, 0xFF), (0x21, 0x55, 0xFF), (0x82, 0x37, 0xFA),
  (0xEB, 0x2F, 0xB5), (0xFF, 0x29, 0x50), (0xFF, 0x22, 0x00), (0xD6, 0x32, 0x00), (0xC4, 0x62, 0x00),
  (0x35, 0x80, 0x00), (0x05, 0x8F, 0x00), (0x00, 0x8A, 0x55), (0x00, 0x99, 0xCC), (0x21, 0x21, 0x21),
  (0x09, 0x09, 0x09), (0x09, 0x09, 0x09), (0xFF, 0xFF, 0xFF), (0x0F, 0xD7, 0xFF), (0x69, 0xA2, 0xFF),
  (0xD4, 0x80, 0xFF), (0xFF, 0x45, 0xF3), (0xFF, 0x61, 0x8B), (0xFF, 0x88, 0x33), (0xFF, 0x9C, 0x12),
  (0xFA, 0xBC, 0x20), (0x9F, 0xE3, 0x0E), (0x2B, 0xF0, 0x35), (0x0C, 0xF0, 0xA4), (0x05, 0xFB, 0xFF),
  (0x5E, 0x5E, 0x5E), (0x0D, 0x0D, 0x0D), (0x0D, 0x0D, 0x0D), (0xFF, 0xFF, 0xFF), (0xA6, 0xFC, 0xFF),
  (0xB3, 0xEC, 0xFF), (0xDA, 0xAB, 0xEB), (0xFF, 0xA8, 0xF9), (0xFF, 0xAB, 0xB3), (0xFF, 0xD2, 0xB0),
  (0xFF, 0xEF, 0xA6), (0xFF, 0xF7, 0x9C), (0xD7, 0xE8, 0x95), (0xA6, 0xED, 0xAF), (0xA2, 0xF2, 0xDA),
  (0x99, 0xFF, 0xFC), (0xDD, 0xDD, 0xDD), (0x11, 0x11, 0x11), (0x11, 0x11, 0x11)
];


// http://www.romdetectives.com/Wiki/index.php?title=NES_Palette
// const PALETTE_TABLE: [(u8, u8, u8); 64] = [
//   (124,124,124),
//   (0,0,252),
//   (0,0,188),
//   (68,40,188),
//   (148,0,132),
//   (168,0,32),
//   (168,16,0),
//   (136,20,0),
//   (80,48,0),
//   (0,120,0),
//   (0,104,0),
//   (0,88,0),
//   (0,64,88),
//   (0,0,0),
//   (0,0,0),
//   (0,0,0),

//   (188,188,188),
//   (0,120,248),
//   (0,88,248),
//   (104,68,252),
//   (216,0,204),
//   (228,0,88),
//   (248,56,0),
//   (228,92,16),
//   (172,124,0),
//   (0,184,0),
//   (0,168,0),
//   (0,168,68),
//   (0,136,136),
//   (0,0,0),
//   (0,0,0),
//   (0,0,0),

//   (248,248,248),
//   (60,188,252),
//   (104,136,252),
//   (152,120,248),
//   (248,120,248),
//   (248,88,152),
//   (248,120,88),
//   (252,160,68),
//   (248,184,0),
//   (184,248,24),
//   (88,216,84),
//   (88,248,152),
//   (0,232,216),
//   (120,120,120),
//   (0,0,0),
//   (0,0,0),

//   (252,252,252),
//   (164,228,252),
//   (184,184,248),
//   (216,184,248),
//   (248,184,248),
//   (248,164,192),
//   (240,208,176),
//   (252,224,168),
//   (248,216,120),
//   (216,248,120),
//   (184,248,184),
//   (184,248,216),
//   (0,252,252),
//   (248,216,248),

//   (0,0,0),
//   (0,0,0)
// ];

#[derive(Copy, Clone)]
struct Sprite {
  x: u8,
  y: u8,
  palette: [u8; 4],
  sprite_behind_background: bool,
  x_flip: bool,
  y_flip: bool,
  tile_high: u8,
  tile_low: u8
}


impl Sprite {
  pub fn new() -> Self {
    Self {
      x: 0,
      y: 0,
      palette: [0; 4],
      sprite_behind_background: false,
      x_flip: false,
      y_flip: false,
      tile_high: 0,
      tile_low: 0,
    }
  }
}

pub struct PPU {
  ctrl: ControlRegister,
  mask: MaskRegister,
  scroll: ScrollRegister,
  status: StatusRegister,
  pub palette_table: [u8; 32],
  pub chr_rom: Vec<u8>,
  pub chr_ram: Vec<u8>,
  pub vram: [u8; 2048],
  pub oam_data: [u8; 256],
  secondary_oam: [u8; 32],
  pub oam_address: u8,
  pub mirroring: Mirroring,
  internal_data: u8,
  cycles: u16,
  current_scanline: u16,
  pub nmi_triggered: bool,
  pub picture: Picture,
  pub joypad: Joypad,
  previous_time: u128,
  pub mapper: Mapper,
  previous_palette: u8,
  current_palette: u8,
  next_palette: u8,
  tile_low: u8,
  tile_high: u8,
  tile_address: u16,
  oam_read: u8,
  secondary_oam_address: u8,
  oam_n: u8,
  oam_m: u8,
  sprites: [Sprite; 8],
  sprite_zero_found: bool,
  tile_shift_high: u16,
  tile_shift_low: u16
}

impl PPU {
  pub fn new(chr_rom: Vec<u8>, chr_ram: Vec<u8>, mirroring: Mirroring) -> Self {
    PPU {
      ctrl: ControlRegister::from_bits_truncate(0b00000000),
      mask: MaskRegister::from_bits_truncate(0b00000000),
      scroll: ScrollRegister::new(),
      status: StatusRegister::from_bits_truncate(0b00000000),
      chr_rom,
      chr_ram,
      oam_data: [0; 256],
      secondary_oam: [0; 32],
      oam_address: 0,
      vram: [0; 2048],
      mirroring,
      internal_data: 0,
      palette_table: [0; 32],
      cycles: 0,
      current_scanline: 0,
      nmi_triggered: false,
      picture: Picture::new(),
      joypad: Joypad::new(),
      previous_time: 0,
      mapper: Mapper::Empty(Empty {}),
      previous_palette: 0,
      current_palette: 0,
      next_palette: 0,
      tile_low: 0,
      tile_high: 0,
      tile_address: 0,
      oam_read: 0,
      oam_m: 0,
      oam_n: 0,
      secondary_oam_address: 0,
      sprites: [Sprite::new(); 8],
      sprite_zero_found: false,
      tile_shift_high: 0,
      tile_shift_low: 0
    }
  }

  pub fn tick(&mut self) {
    self.cycles += 1;

    if self.cycles >= CYCLES_PER_SCANLINE {
      self.cycles -= CYCLES_PER_SCANLINE;

      self.current_scanline += 1;

      if self.current_scanline == SCREEN_HEIGHT+1 {
        self.status.remove(StatusRegister::SPRITE_ZERO_HIT);
        self.status.insert(StatusRegister::VBLANK_STARTED);
        if self.ctrl.generate_nmi_interrupt() {
          self.nmi_triggered = true;
        }
      }

      if self.current_scanline >= SCANLINES_PER_FRAME {
        // self.render();
        self.current_scanline = 0;
        self.nmi_triggered = false;
        self.status.remove(StatusRegister::VBLANK_STARTED);
        self.status.remove(StatusRegister::SPRITE_ZERO_HIT);
      }
    } else {
      self.cycle();
    }
  }

  fn evaluate_sprites(&mut self) {
    match self.cycles {
      1..=64 => {
        // set secondary oam data to ff
        self.secondary_oam.fill(0xff);
        self.oam_read = 0xff;
      }
      65..=256 => {
        if self.cycles == 65 {
          self.secondary_oam_address = 0;
          self.oam_n = 0;
          self.oam_m = 0;
          self.sprite_zero_found = false;
        }
        if self.cycles % 2 == 1 {
          self.oam_read = self.oam_data[self.oam_address as usize];
        } else if ((self.secondary_oam_address + 4) as usize) < self.secondary_oam.len() {
          let y_coordinate = self.oam_data[(self.oam_n * 4) as usize];

          self.secondary_oam[self.secondary_oam_address as usize] = y_coordinate;

          let y_pos_in_tile: i16 = (self.current_scanline as i16) - (y_coordinate as i16);

          if y_pos_in_tile >= 0 && y_pos_in_tile < self.ctrl.sprite_size() as i16 {
            if self.oam_n == 0 {
              self.sprite_zero_found = true;
            }

            // copy the remaining bytes of oam[n][1-3] into secondary oam
            self.secondary_oam[(self.secondary_oam_address + 1) as usize] = self.oam_data[(self.oam_n * 4 + 1) as usize];
            self.secondary_oam[(self.secondary_oam_address + 2) as usize] = self.oam_data[(self.oam_n * 4 + 2) as usize];
            self.secondary_oam[(self.secondary_oam_address + 3) as usize] = self.oam_data[(self.oam_n * 4 + 3) as usize];

            self.secondary_oam_address += 4;
          }
          self.oam_n = (self.oam_n + 1) % 64; // n is in the range 0-63 for evaluating all 64 sprites
          if self.oam_n == 0 {
            // all 64 sprites are evaluated
            self.oam_read = self.oam_data[(self.oam_n * 4) as usize];
            self.oam_n += 1;
          } else if self.secondary_oam_address == self.secondary_oam.len() as u8 {
            // evaluate the rest of the sprites and set the sprite overflow flag
            while self.oam_n != 0 {
              self.oam_m = 0;

              let y_coordinate = self.oam_data[(self.oam_n * 4 + self.oam_m) as usize];

              let y_pos_in_tile = self.current_scanline as i16 - y_coordinate as i16;

              if y_pos_in_tile > 0 && y_pos_in_tile < self.ctrl.sprite_size() as i16 {
                self.status.insert(StatusRegister::SPRITE_OVERFLOW);

                self.oam_m += 1;

                while self.oam_m < 4 {
                  self.oam_read = self.oam_data[(self.oam_n * 4 + self.oam_m) as usize];

                  self.oam_m = (  self.oam_m + 1) % 4;
                }
              } else {
                self.oam_n = (self.oam_n + 1) % 64;
                self.oam_m = (self.oam_m + 1) % 4;
              }
            }
            self.oam_read = self.oam_data[(self.oam_n * 4) as usize];
            self.oam_n += 1;
          }
        }
      },
      _ => ()
    }
  }

  fn fetch_sprites(&mut self) {
    self.write_to_oam_address(0);

    // this is actually an approximation but should be good enough
   if self.cycles % 8 == 4 {
      let sprites_found = self.secondary_oam_address / 4;
      let index = (self.cycles - OAM_FETCH_START) / 8;
      let oam_index = index * 4;

      let y = self.secondary_oam[oam_index as usize];
      let mut tile_number = self.secondary_oam[(oam_index+1) as usize];
      let attributes = self.secondary_oam[(oam_index+2) as usize];
      let x = self.secondary_oam[(oam_index+3) as usize];

      let palette_index = attributes & 0b11;

      let palette = self.get_sprite_palette(palette_index);

      let mut y_pos_in_tile = (self.current_scanline as i16) - (y as i16);

      let y_flip = (attributes >> 7) & 0b1 == 1;
      let x_flip = (attributes >> 6) & 0b1 == 1;

      let sprite_behind_background = (attributes >> 5) & 0b1 == 1;

      if y_flip {
        y_pos_in_tile = self.ctrl.sprite_size() as i16 - 1 - y_pos_in_tile;
      }

      if y_pos_in_tile > 0 && y_pos_in_tile < self.ctrl.sprite_size() as i16 {
        let bank = if self.ctrl.sprite_size() == 8 {
          self.ctrl.sprite_pattern_table_address()
        } else {
          let bank: u16 = if tile_number & 0b1 == 0 { 0 } else { 0x1000 };
          tile_number = tile_number & 0b11111110;

          if y_pos_in_tile > 7 {
            y_pos_in_tile += 8;
          }
          bank
        };

        let tile_index = bank + tile_number as u16 * 16;

        let tile_low = self.read_chr(tile_index + y_pos_in_tile as u16);
        let tile_high = self.read_chr(tile_index + y_pos_in_tile as u16 + 8);

        if index < sprites_found as u16 {
          let mut sprite = &mut self.sprites[index as usize];

          sprite.x_flip = x_flip;
          sprite.y_flip = y_flip;
          sprite.sprite_behind_background = sprite_behind_background;
          sprite.tile_high = tile_high;
          sprite.tile_low = tile_low;
          sprite.palette = palette;
          sprite.x = x;
          sprite.y = y;
        }
      }
   }
  }

  fn fetch_attribute_byte(&mut self) {
    let attribute_address = self.scroll.attribute_address();

    let attribute_byte = self.vram[self.mirror_vram_index(attribute_address) as usize];
    let shift = self.scroll.attribute_shift();

    self.next_palette = 1 + ((attribute_byte >> shift) & 0b11) * 4;
  }

  fn fetch_nametable_byte(&mut self) {
    let address = self.scroll.tile_address();

    self.previous_palette = self.current_palette;
    self.current_palette = self.next_palette;

    let tile_number = self.vram[self.mirror_vram_index(address) as usize];
    let bank = self.ctrl.background_pattern_table_addr();

    self.tile_shift_low |= self.tile_low as u16;
    self.tile_shift_high |= self.tile_high as u16;

    let tile_index = bank + (tile_number as u16) * 16;

    self.tile_address = tile_index + self.scroll.fine_y();
  }

  fn cycle(&mut self) {
    if self.rendering_enabled() {
      if self.current_scanline < SCREEN_HEIGHT {
        // do sprite evaluation only for visible scanlines
        if matches!(self.cycles, 1..=256) {
          self.evaluate_sprites()
        }
      }

      if self.current_scanline < SCREEN_HEIGHT || self.current_scanline == PRERENDER_SCANLINE {
        if matches!(self.cycles, 1..=256) || matches!(self.cycles, 321..=336) {
          // fetch nametable byte, attribute byte, pattern table high and low bytes
          match self.cycles % 8 {
            1 => self.fetch_nametable_byte(),
            3 => self.fetch_attribute_byte(),
            5 => self.tile_low = self.read_chr(self.tile_address),
            7 => self.tile_high = self.read_chr(self.tile_address + 8),
            _ => ()
          }

          if self.cycles % 8 == 0 {
            self.scroll.increment_x();
          }
        } else if matches!(self.cycles, 337..=340) {
          self.fetch_nametable_byte();
        }

        match self.cycles {
          1..=8 if self.current_scanline == PRERENDER_SCANLINE => {
            let address = (self.cycles - 1) as usize;
            self.oam_data[address] = self.oam_data[(self.oam_address as usize & 0xF8) + address]
          },
          256 => self.scroll.increment_y(),
          257 => self.scroll.copy_x(),
          280..=304 if self.current_scanline == PRERENDER_SCANLINE => self.scroll.copy_y(),
          _ => ()
        }

        if matches!(self.cycles, 257..=320) {
          // fetch sprites for next scanline
          self.fetch_sprites();
        }

        if matches!(self.cycles, 321..=340) {
          // sprite dummy cycle...
          self.oam_read = self.secondary_oam[0];
        }
      }
    }

    // finally render the pixel
    if matches!(self.cycles, 1..=256) && self.current_scanline < SCREEN_HEIGHT {
      self.draw_pixel();
    }
    if matches!(self.cycles, 1..=256) {
      self.tile_shift_high <<= 1;
      self.tile_shift_low <<= 1;
    }

  }

  pub fn draw_pixel(&mut self) {
    let x = self.cycles - 1;
    let y = self.current_scanline;

    let is_left_bg_clipped = x < 8 && !self.mask.contains(MaskRegister::SHOW_BACKGROUND_LEFTMOST);

    let bg_color = if self.mask.contains(MaskRegister::SHOW_BACKGROUND) && !is_left_bg_clipped {
      let offset = self.scroll.fine_x();
      (((self.tile_shift_high << offset) & 0x8000) >> 14) + (((self.tile_shift_low << offset) & 0x8000) >> 15)
    } else {
      0
    };

    let is_left_sprite_clipped = x < 8 && !self.mask.contains(MaskRegister::SHOW_SPRITES_LEFTMOST);

    let mut rgb: Option<(u8, u8, u8)> = None;

    if self.mask.contains(MaskRegister::SHOW_SPRITES) && !is_left_sprite_clipped {
      let found_sprite_count = self.secondary_oam_address / 4;

      for i in 0..found_sprite_count {
        let sprite = &self.sprites[i as usize];

        let mut bit_pos = x as i16 - sprite.x as i16;

        if !sprite.x_flip {
          bit_pos = 7 - bit_pos;
        }

        if (0..8).contains(&bit_pos) {
          let color_index = ((sprite.tile_low >> bit_pos) & 0b1) + (((sprite.tile_high >> bit_pos) & 0b1) << 1);
          if i == 0
            && color_index != 0
            && self.sprite_zero_found
            && x != 255
            && self.rendering_enabled()
            && !self.status.contains(StatusRegister::SPRITE_ZERO_HIT) {
              self.status.insert(StatusRegister::SPRITE_ZERO_HIT);
            }

            rgb = if color_index != 0 && (bg_color == 0 || !sprite.sprite_behind_background) {
              let palette_index = sprite.palette[color_index as usize];

              Some(PALETTE_TABLE[palette_index as usize])
            } else {
              None
            };
        }
      }
      let actual_rgb = if let Some(rgb) = rgb {
        rgb
      } else {
        let palette_search = if (self.scroll.fine_x() + (x as u8 % 8)) < 8 {
          self.previous_palette
        } else {
          self.current_palette
        };

        let palette = self.get_bg_palette(palette_search as usize);

        let palette_index = palette[bg_color as usize];

        PALETTE_TABLE[palette_index as usize]
      };

      self.picture.set_pixel(x as usize, y as usize, actual_rgb);
    }
  }

  pub fn update_mirroring(&mut self) {
    if !matches!(self.mapper, Mapper::Empty(_)) {
      self.mirroring = self.mapper.mirroring()
    }
  }

  pub fn cap_fps(&mut self) {
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

    self.previous_time = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("an error occurred")
      .as_millis();
  }

  // see https://www.nesdev.org/wiki/Mirroring
  fn mirror_vram_index(&self, address: u16) -> u16 {
    let mirrored_address = address & 0b10111111111111; // mirror down address to range 0x2000 to 0x2eff, where nametables exist
    let vram_index = mirrored_address - 0x2000;
    let name_table_index = vram_index / 0x400; // this should give us a value between 0-3 which points to what nametable is being referred to

    match (&self.mirroring, name_table_index) {
      (Mirroring::SingleScreenA, 3) => vram_index - 0xc00,
      (Mirroring::SingleScreenB, 0) => vram_index + 0x400,
      (Mirroring::Horizontal, 1)
        | (Mirroring::Horizontal, 2)
        | (Mirroring::SingleScreenA, 1)
        | (Mirroring::SingleScreenB, 2) => vram_index - 0x400, // for horizontal, 1 is in first kb of memory, 2 is in 2nd kb of memory.
      (Mirroring::Vertical, 2)
        | (Mirroring::Vertical, 3)
        | (Mirroring::Horizontal, 3)
        | (Mirroring::SingleScreenB, 3)
        | (Mirroring::SingleScreenA, 2) => vram_index- 0x800, // 2 is in first kb of memory 3 is in 2nd for vertical. 3 is in 2nd for horizontal.
      _ => vram_index // either it's four screen which has no mirroring or it's nametable 0 or another nametable that doesn't need the offset
    }
  }

  pub fn read_status_register(&mut self) -> u8 {
    let data = self.status.bits();

    self.scroll.latch = false;
    self.status.set(StatusRegister::VBLANK_STARTED, false);

    data
  }

  fn read_chr(&mut self, address: u16) -> u8 {

    let chr = if !self.chr_rom.is_empty() {
      &self.chr_rom
    } else {
      &self.chr_ram
    };

    match &mut self.mapper {
      Mapper::Empty(_) => chr[address as usize],
      _ => {
        if let Some(mapped_address) = self.mapper.mem_read(address) {
          chr[mapped_address]
        } else {
          0
        }
      }
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

  fn get_bg_palette(&self, palette_start: usize) -> [u8; 4] {
    [self.palette_table[0], self.palette_table[palette_start], self.palette_table[palette_start + 1], self.palette_table[palette_start + 2]]
  }

  pub fn read_oam_data(&self) -> u8 {
    if self.current_scanline < SCREEN_HEIGHT && self.rendering_enabled() && matches!(self.cycles, 257..=320) {
      self.secondary_oam[(self.oam_n * 4 + self.oam_m) as usize]
    } else {
      self.oam_data[self.oam_address as usize]
    }
  }

  pub fn write_to_control(&mut self, value: u8) {
    let tmp = self.ctrl.generate_nmi_interrupt();
    self.ctrl = ControlRegister::from_bits_truncate(value);
    if !tmp && self.status.contains(StatusRegister::VBLANK_STARTED) && self.ctrl.generate_nmi_interrupt() {
      self.nmi_triggered = true;
    }
    self.scroll.set_nametable_select(value);
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
    self.scroll.set_scroll(value);
  }

  pub fn write_to_ppu_address(&mut self, value: u8) {
    self.scroll.set_address(value);
  }

  pub fn write_to_data(&mut self, value: u8) {
    let address = self.scroll.get_address();

    match address {
      0x0000 ..= 0x1fff => {
        if !self.chr_ram.is_empty() {
          self.chr_ram[address as usize] = value;
        }
      },
      0x2000 ..=0x2fff => self.vram[self.mirror_vram_index(address) as usize] = value,
      0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
        let address_mirror = address - 0x10;
        self.palette_table[((address_mirror - 0x3f00) % self.palette_table.len() as u16) as usize] = value;
      }
      0x3f00 ..=0x3fff => self.palette_table[((address - 0x3f00) % self.palette_table.len() as u16) as usize] = value,
      _ => panic!("shouldn't get here")
    }

    self.mapper.ppu_bus_write(address, value);

    self.increment_address(self.ctrl.vram_address_increment());
  }

  fn increment_address(&mut self, val: u8) {
    if self.rendering_enabled() && self.current_scanline < SCREEN_WIDTH && self.current_scanline == PRERENDER_SCANLINE {
      self.scroll.increment_x();
      self.scroll.increment_y();
    } else {
      self.scroll.increment_address(val);
    }
  }

  fn rendering_enabled(&self) -> bool {
    self.mask.contains(MaskRegister::SHOW_SPRITES) || self.mask.contains(MaskRegister::SHOW_BACKGROUND)
  }

  pub fn read_data(&mut self) -> u8 {
    let address = self.scroll.get_address();

    self.increment_address(self.ctrl.vram_address_increment());

    match address {
      0x0000 ..= 0x1fff => {
        let result = self.internal_data;

        self.internal_data = self.read_chr(address);

        result
      },
      0x2000 ..= 0x2fff => {
        let result = self.internal_data;

        self.internal_data = self.vram[self.mirror_vram_index(address) as usize];

        result
      }
      0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
        let address_mirror = address - 0x10;

        self.internal_data = self.vram[self.mirror_vram_index(address - 0x1000) as usize];

        self.palette_table[((address_mirror - 0x3f00) % self.palette_table.len() as u16) as usize]
      }

      0x3f00..=0x3fff =>
      {
        self.internal_data = self.vram[self.mirror_vram_index(address - 0x1000) as usize];

        self.palette_table[((address - 0x3f00) % self.palette_table.len() as u16) as usize]
      }
      _ => panic!("shouldn't get here")
    }
  }
}