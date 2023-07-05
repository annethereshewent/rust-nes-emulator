pub mod sxrom;

use sxrom::Sxrom;

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

  fn tick(&mut self, cycles: u8) {

  }
}

impl MapperActions for Mapper {
  fn mem_read(&mut self, _address: u16) -> Option<usize> {
    None
  }

  fn mem_write(&mut self, _address: u16, _val: u8) -> Option<usize> {
    None
  }

  fn tick(&mut self, cycles: u8) {

  }
}

pub struct Empty { }

impl MapperActions for Empty { }