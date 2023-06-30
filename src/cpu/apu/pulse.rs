
use super::LENGTH_TABLE;
use super::envelope::Envelope;
use super::registers::control::Control;
use super::registers::sweep::Sweep;
use super::registers::timer_low::TimerLow;
use super::registers::timer_high::TimerHigh;

pub enum PulseChannel {
  One,
  Two
}

pub struct Pulse {
  pub control: Control,
  pub sweep: Sweep,
  pub timer_low: TimerLow,
  pub timer_high: TimerHigh,
  frequency_counter: i16,
  duty_counter: u8,
  envelope: Envelope,
  length_counter: u8,
  channel: PulseChannel,
  enabled: bool
}

const DUTY_TABLE: [[u8; 8]; 4] = [
  [0,1,0,0,0,0,0,0],
  [0,1,1,0,0,0,0,0],
  [0,1,1,1,1,0,0,0],
  [1,0,0,1,1,1,1,1]
];

impl Pulse {
  pub fn new(channel: PulseChannel) -> Self {
    let control = Control::new();

    Self {
      control,
      sweep: Sweep::new(),
      timer_low: TimerLow::new(),
      timer_high: TimerHigh::new(),
      frequency_counter: 0,
      duty_counter: 0,
      envelope: Envelope::new(),
      length_counter: 0,
      channel,
      enabled: true
    }
  }

  pub fn tick(&mut self, cycles: u16) {
    if self.frequency_counter > 0 {
      self.frequency_counter -= cycles as i16;
    } else {
      self.frequency_counter = self.timer() as i16;

      self.duty_counter = (self.duty_counter + 1) % 8
    }
  }

  fn timer(&self) -> u16 {
    (self.timer_high.timer_bits() as u16) << 8 | self.timer_low.bits() as u16
  }

  pub fn clock_quarter_frame(&mut self) {
    self.envelope.clock(&self.control);
  }

  pub fn clock_half_frame(&mut self) {
    if self.sweep.reload {
      self.sweep.reload = false;
      self.sweep.counter = self.sweep.divider_period();
    } else if self.sweep.counter > 0 {
      self.sweep.counter -= 1;
    } else {
      self.sweep.counter = self.sweep.divider_period();

      if self.sweep.enabled() && !self.sweep_force_silence() {
        let delta = self.timer() >> self.sweep.shift_count();

        if self.sweep.negate_flag() {
          self.set_timer(self.timer() - delta + 1);
          if matches!(self.channel, PulseChannel::One) {
            self.set_timer(self.timer() + 1);
          }
        } else {
          self.set_timer(self.timer() + delta)
        }
      }
    }

    self.clock_length_counter();
  }

  fn clock_length_counter(&mut self) {
    if self.control.length_counter_halt() == 0 && self.length_counter > 0 {
      self.length_counter -= 1;
    }
  }

  fn set_timer(&mut self, new_timer: u16) {
    self.timer_low.set((new_timer & 0b11111111) as u8) ;
    self.timer_high.set(((new_timer >> 8) & 0b111) as u8);
  }

  fn sweep_force_silence(&self) -> bool {
    let next_freq = self.timer() + (self.timer() >> self.sweep.shift_count());

    self.timer() < 8 || (!self.sweep.negate_flag() && next_freq >= 0x800)
  }

  pub fn write_timer_high(&mut self, val: u8) {
    self.timer_high.set(val);

    self.frequency_counter = self.timer() as i16;

    self.duty_counter = 0;

    self.envelope.reset = true;

    if self.enabled {
      self.length_counter = LENGTH_TABLE[(val >> 3) as usize];
    }

  }

  pub fn toggle(&mut self, enabled: bool) {
    self.enabled = enabled;
    if !enabled {
      self.length_counter = 0;
    }
  }

  pub fn output(&self) -> f32 {
    if DUTY_TABLE[self.control.duty_cycle() as usize][self.duty_counter as usize] != 0
     && self.length_counter != 0
     && !self.sweep_force_silence()
    {
      if self.control.constant_vol_env_flag() == 0 {
        self.envelope.volume as f32
      } else {
        self.control.envelope_divider_period() as f32
      }
    } else {
      0 as f32
    }

  }
}