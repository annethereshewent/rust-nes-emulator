
bitflags! {
  pub struct Status: u8 {
    const PULSE1_ENABLE   = 0b1;
    const PULSE2_ENABLE   = 0b10;
    const TRIANGLE_ENABLE = 0b100;
    const NOISE_ENABLE    = 0b1000;
    const DMC_ENABLE      = 0b10000;
  }
}