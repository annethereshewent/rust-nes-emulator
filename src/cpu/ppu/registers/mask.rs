bitflags! {
  #[derive(Clone, Copy)]
  pub struct MaskRegister: u8 {
    const GRAYSCALE                = 0b1;
    const SHOW_BACKGROUND_LEFTMOST = 0b10;
    const SHOW_SPRITES_LEFTMOST    = 0b100;
    const SHOW_BACKGROUND          = 0b1000;
    const SHOW_SPRITES             = 0b10000;
    const EMPHASIZE_RED            = 0b100000;
    const EMPHASIZE_GREEN          = 0b1000000;
    const EMPHASIZE_BLUE           = 0b10000000;
  }
}

impl MaskRegister {
  pub fn emphasis(&mut self) -> u8 {
    self.intersection(Self::EMPHASIZE_RED | Self::EMPHASIZE_GREEN | Self::EMPHASIZE_BLUE).bits()
  }
}