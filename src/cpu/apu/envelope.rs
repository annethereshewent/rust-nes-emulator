pub struct Envelope {
    counter: u8,
    pub volume: u8,
    pub reset: bool,
}

impl Envelope {
  pub fn new() -> Self {
    Self {
      counter: 0,
      volume: 0,
      reset: false
    }
  }

  pub fn clock(&mut self, envelope_divider_period: u8, length_counter_halt: u8) {
    if self.reset {
      self.reset = false;
      self.volume = 0xf;
      self.counter = envelope_divider_period;
    } else if self.counter > 0 {
      self.counter -= 1;
    } else {
      self.counter = envelope_divider_period;
      if self.volume > 0 {
        self.volume -= 1;
      } else if length_counter_halt == 1 {
        self.volume = 0xf;
      }
    }
  }
}