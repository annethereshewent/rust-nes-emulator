pub mod sxrom;

use sxrom::Sxrom;

pub enum Mapper {
  Empty(Empty),
  Sxrom(Sxrom)
}

pub trait MapperActions {
  fn mem_read(&mut self, address: u16) -> Option<usize> {
    None
  }

  fn mem_write(&mut self, address: u16, val: u8) -> Option<usize> {
    None
  }

  fn tick(&mut self) {

  }
}

impl MapperActions for Mapper {
  fn mem_read(&mut self, address: u16) -> Option<usize> {
    None
  }

  fn mem_write(&mut self, address: u16, val: u8) -> Option<usize> {
    None
  }

  fn tick(&mut self) {

  }
}

pub struct Empty { }

impl MapperActions for Empty { }