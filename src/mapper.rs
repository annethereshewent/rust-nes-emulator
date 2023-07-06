pub mod sxrom;

use sxrom::Sxrom;

use crate::cartridge::Mirroring;

pub enum Mapper {
  Empty(Empty),
  Sxrom(Sxrom)
}

pub trait MapperActions {
  fn mem_read(&mut self, _address: u16) -> Option<usize> {
    None
  }

  fn mem_write(&mut self, _address: u16, _val: u8) -> Option<usize> {
    None
  }

  fn tick(&mut self, _cycles: u8) {

  }

  fn mirroring(&mut self) -> Mirroring {
    Mirroring::SingleScreenA
  }
}

impl MapperActions for Mapper {
  fn mem_read(&mut self, address: u16) -> Option<usize> {
    match self {
      Mapper::Empty(_) => None,
      Mapper::Sxrom(sxrom) => sxrom.mem_read(address)
    }
  }

  fn mem_write(&mut self, address: u16, val: u8) -> Option<usize> {
    match self {
      Mapper::Empty(_) => None,
      Mapper::Sxrom(sxrom) => sxrom.mem_write(address, val)
    }
  }

  fn tick(&mut self, cycles: u8) {
    match self {
      Mapper::Empty(_) => (),
      Mapper::Sxrom(sxrom) => sxrom.tick(cycles)
    }
  }

  fn mirroring(&mut self) -> Mirroring {
    match self {
      Mapper::Sxrom(sxrom) => sxrom.mirroring,
      _ => panic!("mapper not supported")
    }
  }
}

pub struct Empty { }

impl MapperActions for Empty { }