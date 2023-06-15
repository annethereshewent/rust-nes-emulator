pub mod registers;

use registers::control::ControlRegister;
use registers::mask::MaskRegister;
use registers::scroll::ScrollRegister;
use registers::status::StatusRegister;
use self::registers::address::AddressRegister;

use crate::nes::cartridge::Mirroring;

pub struct PPU {
  ctrl: ControlRegister,
  mask: MaskRegister,
  scroll: ScrollRegister,
  status: StatusRegister,
  ppu_addr: AddressRegister,
  pub chr_rom: Vec<u8>,
  pub vram: [u8; 2048],
  pub oam_data: [u8; 256],
  pub oam_address: u8,
  pub mirroring: Mirroring,
  internal_data: u8
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
      internal_data: 0
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

  pub fn read_data(&mut self) -> u8 {
    let address = self.ppu_addr.get();

    self.ppu_addr.increment(self.ctrl.vram_address_increment());


    self.oam_data[address as usize  ]
  }
}