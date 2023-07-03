pub mod mmc1;

use mmc1::Mmc1;

pub enum Mapper {
  Empty(Empty),
  Mmc1(Mmc1)
}

pub trait MapperActions {
  fn map_read(&mut self, address: u16) -> u16 {
    0
  }

  fn map_write(&mut self, address: u16, val: u8) -> Option<u16> {
    None
  }

  fn clock(&mut self) {

  }
}

impl MapperActions for Mapper {
  fn map_read(&mut self, address: u16) -> u16 {
    0
  }

  fn map_write(&mut self, address: u16, val: u8) -> Option<u16> {
    None
  }

  fn clock(&mut self) {

  }
}

pub struct Empty { }

impl MapperActions for Empty { }