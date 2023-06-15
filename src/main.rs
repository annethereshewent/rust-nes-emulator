pub mod nes;

use std::env;
use nes::NES;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

fn main() {
  let mut nes = NES::new();

  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Please specify a filename.");
  }

  let filepath = &args[1];

  nes.run(filepath);
}