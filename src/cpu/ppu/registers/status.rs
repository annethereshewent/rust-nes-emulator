bitflags! {
  pub struct StatusRegister: u8 {
    const SPRITE_OVERFLOW = 0b100000;
    const SPRITE_ZERO_HIT = 0b1000000;
    const VBLANK_STARTED  = 0b10000000;
  }
}
