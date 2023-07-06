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
use self::registers::address::AddressRegister;

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

pub struct PPU {
  ctrl: ControlRegister,
  mask: MaskRegister,
  scroll: ScrollRegister,
  status: StatusRegister,
  ppu_addr: AddressRegister,
  pub palette_table: [u8; 32],
  pub chr_rom: Vec<u8>,
  pub chr_ram: Vec<u8>,
  pub vram: [u8; 2048],
  pub oam_data: [u8; 256],
  pub oam_address: u8,
  pub mirroring: Mirroring,
  internal_data: u8,
  cycles: u16,
  current_scanline: u16,
  pub nmi_triggered: bool,
  pub picture: Picture,
  pub joypad: Joypad,
  background_pixels_drawn: Vec<bool>,
  previous_time: u128,
  pub mapper: Mapper
}

impl PPU {
  pub fn new(chr_rom: Vec<u8>, chr_ram: Vec<u8>, mirroring: Mirroring) -> Self {
    PPU {
      ctrl: ControlRegister::from_bits_truncate(0b00000000),
      mask: MaskRegister::from_bits_truncate(0b00000000),
      scroll: ScrollRegister::new(),
      status: StatusRegister::from_bits_truncate(0b00000000),
      ppu_addr: AddressRegister::new(),
      chr_rom,
      chr_ram,
      oam_data: [0; 256],
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
      background_pixels_drawn: Vec::new(),
      previous_time: 0,
      mapper: Mapper::Empty(Empty {})
    }
  }

  pub fn tick(&mut self, cycles: u16) {
    self.cycles += cycles;

    if self.cycles >= CYCLES_PER_SCANLINE {
      if self.current_scanline < SCREEN_HEIGHT {
        self.draw_line();
      }
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
    self.ppu_addr.latch = false;
    self.status.set(StatusRegister::VBLANK_STARTED, false);

    data
  }

  fn draw_line(&mut self) {
    self.background_pixels_drawn = Vec::new();
    self.draw_background();
    self.draw_sprites();
  }

  fn sprite_zero_hit(&self, i: usize, x: usize) -> bool {
    i == 0 &&
    x != 255 &&
    self.mask.contains(MaskRegister::SHOW_SPRITES) &&
    !self.status.contains(StatusRegister::SPRITE_ZERO_HIT)
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

        let lower_byte = self.read_chr(tile_index + y_pos_in_tile as u16);
        let upper_byte = self.read_chr(tile_index + y_pos_in_tile as u16 + 8);

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

          if x_pos >= SCREEN_WIDTH as usize || x_pos < 0 as usize {
            continue;
          }

          if self.sprite_zero_hit(i, x_pos) {
            self.status.set(StatusRegister::SPRITE_ZERO_HIT, true);
          }

          let is_pixel_visible = !(sprite_behind_background && self.background_pixels_drawn[x_pos]);

          if is_pixel_visible {
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
      (_, Mirroring::SingleScreenA) => 0x2000,
      (_, Mirroring::SingleScreenB) => 0x2400,
      _ => todo!("mirroring mode not implemented")
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
      } else if matches!(&self.mirroring, Mirroring::SingleScreenA) | matches!(&self.mirroring, Mirroring::SingleScreenB) {
        nametable_base
      } else {
        todo!("four screen not implemented")
      };

      let tile_pos = (scrolled_x / 8) + (scrolled_y / 8) * 32;

      let tile_number = self.vram[self.mirror_vram_index(current_nametable + tile_pos) as usize];

      let tile_index = chr_rom_bank + (tile_number as u16 * 16);

      let x_pos_in_tile = scrolled_x % 8;
      let y_pos_in_tile = scrolled_y % 8;

      let lower_byte = self.read_chr(tile_index + y_pos_in_tile);
      let upper_byte = self.read_chr(tile_index + y_pos_in_tile + 8);

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

    self.ppu_addr.increment(self.ctrl.vram_address_increment());
  }

  pub fn read_data(&mut self) -> u8 {
    let address = self.ppu_addr.get();

    self.ppu_addr.increment(self.ctrl.vram_address_increment());

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