pub mod sxrom;
pub mod uxrom;
pub mod cnrom;
pub mod txrom;

use sxrom::Sxrom;
use uxrom::Uxrom;
use cnrom::Cnrom;
use txrom::Txrom;

use crate::cartridge::Mirroring;

pub enum Mapper {
  Empty(Empty),
  Sxrom(Sxrom),
  Uxrom(Uxrom),
  Cnrom(Cnrom),
  Txrom(Txrom)
}

pub enum BankType {
  Chr,
  Prg
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

  fn mirroring(&self) -> Mirroring {
    Mirroring::SingleScreenA
  }

  fn ppu_bus_write(&mut self, _address: u16, _val: u8) {

  }

  fn irq_pending(&self) -> bool {
    false
  }

  fn set_irq_pending(&mut self, _val: bool) {

  }
}

impl MapperActions for Mapper {
  fn mem_read(&mut self, address: u16) -> Option<usize> {
    match self {
      Mapper::Empty(_) => None,
      Mapper::Sxrom(sxrom) => sxrom.mem_read(address),
      Mapper::Uxrom(uxrom) => uxrom.mem_read(address),
      Mapper::Cnrom(cnrom) => cnrom.mem_read(address),
      Mapper::Txrom(txrom) => txrom.mem_read(address)
    }
  }

  fn mem_write(&mut self, address: u16, val: u8) -> Option<usize> {
    match self {
      Mapper::Empty(_) => None,
      Mapper::Sxrom(sxrom) => sxrom.mem_write(address, val),
      Mapper::Uxrom(uxrom) => uxrom.mem_write(address, val),
      Mapper::Cnrom(cnrom) => cnrom.mem_write(address, val),
      Mapper::Txrom(txrom) => txrom.mem_write(address, val)
    }
  }

  fn tick(&mut self, cycles: u8) {
    match self {
      Mapper::Sxrom(sxrom) => sxrom.tick(cycles),
      _ => ()
    }
  }

  fn mirroring(&self) -> Mirroring {
    match self {
      Mapper::Sxrom(sxrom) => sxrom.mirroring(),
      Mapper::Uxrom(uxrom) => uxrom.mirroring(),
      Mapper::Cnrom(cnrom) => cnrom.mirroring(),
      Mapper::Txrom(txrom) => txrom.mirroring(),
      _ => panic!("mapper not supported")
    }
  }

  fn ppu_bus_write(&mut self, address: u16, _val: u8) {
    match self {
      Mapper::Txrom(txrom) => txrom.ppu_bus_write(address, _val),
      _ => ()
    }
  }
  fn irq_pending(&self) -> bool {
    match self {
      Mapper::Txrom(txrom) => txrom.irq_pending(),
      _ => false
    }
  }

  fn set_irq_pending(&mut self, val: bool) {
    match self {
      Mapper::Txrom(txrom) => txrom.set_irq_pending(val),
      _ => ()
    }
  }
}

pub struct Empty { }

impl MapperActions for Empty { }