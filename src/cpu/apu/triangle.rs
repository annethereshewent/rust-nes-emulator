use super::{registers::{linear_counter::LinearCounter, timer_low::TimerLow, timer_high::TimerHigh}, LENGTH_TABLE};

pub struct Triangle {
  pub timer_low: TimerLow,
  pub timer_high: TimerHigh,
  linear_counter: LinearCounter,
  length_counter: u8,
  length_enabled: bool,
  frequency_counter: i16,
  enabled: bool,
  is_ultrasonic: bool,
  step: u8
}

impl Triangle {
  pub fn new() -> Self {
    Self {
      linear_counter: LinearCounter::new(),
      timer_low: TimerLow::new(),
      timer_high: TimerHigh::new(),
      length_counter: 0,
      length_enabled: false,
      frequency_counter: 0,
      enabled: false,
      is_ultrasonic: false,
      step: 0
    }
  }

  pub fn tick(&mut self, cycles: u16) {
    self.is_ultrasonic = false;
    if self.length_counter > 0 && self.timer() < 2 && self.frequency_counter == 0 {
      self.is_ultrasonic = true;
    }

    if self.length_counter != 0 && self.linear_counter.counter != 0 && !self.is_ultrasonic {
      if self.frequency_counter > 0 {
        self.frequency_counter -= cycles as i16;
      } else {
        self.frequency_counter += self.timer() as i16;
        self.step = (self.step + 1) & 0x1f;
      }
    }
  }

  pub fn output(&self) -> f32 {
    if self.is_ultrasonic {
      0.0
    } else if self.step >> 5 == 1 {
      (self.step ^ 0x1f) as f32
    } else {
      self.step as f32
    }
  }

  pub fn toggle(&mut self, enabled: bool) {
    self.enabled = enabled;
    if !enabled {
      self.length_counter = 0;
    }
  }

  pub fn clock_length_counter(&mut self) {
    if self.length_enabled && self.length_counter > 0 {
      self.length_counter -= 1;
    }
  }

  pub fn clock_quarter_frame(&mut self) {
    if self.linear_counter.reload {
      self.linear_counter.counter = self.linear_counter.counter_reload();
    } else if self.linear_counter.counter > 0 {
      self.linear_counter.counter -= 1;
    }
    if self.linear_counter.control_flag() == 0 {
      self.linear_counter.reload = false;
    }
  }

  pub fn clock_half_frame(&mut self) {
    self.clock_length_counter();
  }

  pub fn write_linear_counter(&mut self, val: u8) {
    self.linear_counter.set(val);
    self.length_enabled = val >> 7 == 0;
  }

  pub fn write_timer_high(&mut self, val: u8) {
    self.timer_high.set(val);
    self.frequency_counter = self.timer() as i16;
    self.linear_counter.reload = true;

    if self.enabled {
      self.length_counter = LENGTH_TABLE[(val >> 3) as usize];
    }
  }

  fn timer(&self) -> u16 {
    (self.timer_high.timer_bits() as u16) << 8 | self.timer_low.bits() as u16
  }
}