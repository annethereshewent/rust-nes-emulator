pub mod registers;

use registers::control::ControlRegister;
use registers::mask::MaskRegister;
use registers::scroll::ScrollRegister;
use registers::status::StatusRegister;
use self::registers::address::AddressRegister;

use crate::nes::cartridge::Mirroring;

const SCANLINES_PER_FRAME: u16 = 262;
const CYCLES_PER_SCANLINE: u16 = 341;

const SCREEN_HEIGHT: u16 = 241;

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
  pub nmi_triggered: bool
}

impl PPU {
  pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
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
      nmi_triggered: false
    }
  }

  pub fn tick(&mut self, cycles: u16) {
    self.cycles += cycles;

    if self.cycles >= CYCLES_PER_SCANLINE {
      self.cycles -= CYCLES_PER_SCANLINE;

      self.current_scanline += 1;

      if self.current_scanline == SCREEN_HEIGHT {
        // trigger NMI interrupt
        if self.ctrl.generate_nmi_interrupt() {
          self.status.insert(StatusRegister::VBLANK_STARTED);
          self.nmi_triggered = true;
        }
      }

      if self.current_scanline == SCANLINES_PER_FRAME {
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
    let name_table_index = vram_index / 0x400; // this should give us a value between 0-3 which points to what quadrant (or screen) is being referred to

    match (&self.mirroring, name_table_index) {
      (Mirroring::HORIZONTAL, 1) => vram_index - 0x400, // first kb of memory
      (Mirroring::HORIZONTAL, 2) => vram_index - 0x400, // 2nd kb of memory
      (Mirroring::HORIZONTAL, 3) => vram_index - 0x800, // 2nd kb of memory
      (Mirroring::VERTICAL, 2) | (Mirroring::VERTICAL, 3) => vram_index- 0x800, // 2 is in first kb of memory 3 is in 2nd (ie: if vram index is 0xc00 [index 2], subtracting 0x800 would put it at 0x400, start of 2nd kb of ram)
      _ => vram_index // leave as is
    }
  }

  pub fn read_status_register(&mut self) -> u8 {
    let data = self.status.bits();

    self.scroll.latch = false;
    self.ppu_addr.latch = false;
    self.status.set(StatusRegister::VBLANK_STARTED, false);

    data
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