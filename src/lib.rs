#![crate_name = "nes_emulator"]

pub mod cpu;
pub mod cartridge;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
