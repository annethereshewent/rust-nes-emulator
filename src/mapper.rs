pub mod mmc1;

use mmc1::Mmc1;

pub enum Mapper {
  None,
  Mmc1(Mmc1)
}

pub enum MappedRead {
  None,
  Chr(usize),
  PrgRom(usize),
  PrgRam(usize),
}

pub enum MappedWrite {
  None,
  Chr(usize, u8),
  PrgRam(usize, u8)
}

pub trait MemMap {
  fn map_read(&mut self, address: u16) -> MappedRead {
    MappedRead::None
  }

  fn map_write(&mut self, address: u16, val: u8) -> MappedWrite {
    MappedWrite::None
  }
}

impl MemMap for Mapper {
  fn map_read(&mut self, address: u16) -> MappedRead {
    MappedRead::None
  }
}