use super::registers::control::Control;

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

  pub fn clock(&mut self, control: &Control) {
    if self.reset {
      self.reset = false;
      self.volume = 0xf;
      self.counter = control.envelope_divider_period();
    } else if self.counter > 0 {
      self.counter -= 1;
    } else {
      self.counter = control.envelope_divider_period();
      if self.volume > 0 {
        self.volume -= 1;
      } else if control.length_counter_halt() == 1 {
        self.volume = 0xf;
      }
    }
  }
}