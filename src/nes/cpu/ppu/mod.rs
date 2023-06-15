pub mod registers;

use registers::control::ControlRegister;

pub struct PPU {
  control: ControlRegister
}