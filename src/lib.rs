#![crate_name = "nes_emulator"]

pub mod cpu;
pub mod cartridge;
pub mod mapper;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
