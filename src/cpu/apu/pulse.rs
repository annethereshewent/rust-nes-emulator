
use super::registers::control::Control;
use super::registers::sweep::Sweep;
use super::registers::timer_low::TimerLow;
use super::registers::timer_high::TimerHigh;

pub struct Pulse {
  pub control: Control,
  pub sweep: Sweep,
  pub timer_low: TimerLow,
  pub timer_high: TimerHigh
}

impl Pulse {
  pub fn new() -> Self {
    Self {
      control: Control::new(),
      sweep: Sweep::new(),
      timer_low: TimerLow::new(),
      timer_high: TimerHigh::new()
    }
  }

  pub fn tick(&mut self, cycles: u16) {

  }

  pub fn clock_quarter_frame(&mut self) {

  }

  pub fn clock_half_frame(&mut self) {

  }

  pub fn toggle(&mut self, enabled: bool) {

  }
}