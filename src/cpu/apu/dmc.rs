use super::registers::flags_rate_register::FlagsRateRegister;

// see https://www.nesdev.org/wiki/APU_DMC
const FREQUENCY_TABLE: [i16; 16] = [
  0x1ac, 0x17c, 0x154, 0x140, 0x11e, 0x0fe,
  0x0e2, 0x0d6, 0x0be, 0x0a0, 0x08e, 0x080,
  0x06a, 0x054, 0x048, 0x036
];

const TIMER_TABLE: [i16; 16] = [
  428, 380, 340, 320, 286, 254, 226, 214,
  190, 160, 142, 128, 106,  84,  72,  54
];

pub struct DMC {
  pub flags_rate_register: FlagsRateRegister,
  pub direct_load: u8,
  pub dma_pending: bool,
  pub sample_address: u16,
  sample_address_load: u16,
  sample_length_load: u8,
  sample_length: u8,
  frequency_counter: i16,
  output_silent: bool,
  output_bits: u8,
  output_shift: u8,
  sample_buffer_empty: bool,
  sample_buffer: u8,
  init: i8,
  enabled: bool,
  pub irq_pending: bool
}

impl DMC {
  pub fn new() -> Self {
    Self {
      flags_rate_register: FlagsRateRegister::new(),
      direct_load: 0,
      sample_address: 0,
      sample_address_load: 0,
      sample_length_load: 0,
      frequency_counter: 0,
      output_silent: false,
      output_shift: 0,
      output_bits: 0,
      sample_buffer_empty: false,
      sample_buffer: 0,
      dma_pending: false,
      sample_length: 0,
      init: 0,
      enabled: false,
      irq_pending: false
    }
  }

  pub fn tick(&mut self, cycles: u16) {
    if self.frequency_counter > 0 {
      self.frequency_counter -= (cycles * 2) as i16
    } else {
      self.frequency_counter += self.timer();

      if !self.output_silent {
        if self.output_shift & 0b1 == 1 {
          if self.direct_load <= 125 {
            self.direct_load += 2;
          }
        } else if self.direct_load >= 2 {
          self.direct_load -= 2;
        }

        self.output_shift >>= 1;
      }

      self.output_bits = self.output_bits.saturating_sub(1);

      if self.output_bits == 0 {
        self.output_bits = 8;

        if self.sample_buffer_empty {
          self.output_silent = true;
        } else {
          self.output_silent = false;
          self.output_shift = self.sample_buffer;
          self.sample_buffer_empty = true;
          if self.sample_length > 0 {
            self.dma_pending = true;
          }
        }
      }
    }
  }

  pub fn output(&self) -> f32 {
    self.direct_load as f32
  }

  pub fn timer(&self) -> i16 {
    FREQUENCY_TABLE[self.flags_rate_register.rate_index() as usize] - 2
  }

  pub fn toggle(&mut self, enabled: bool, cycle: usize) {
    self.irq_pending = false;
    self.enabled = enabled;

    if !enabled {
      self.sample_length = 0;
    } else if self.sample_length == 0 {
      self.sample_address = self.sample_address_load;
      self.sample_length = self.sample_length_load;

      self.init = if cycle % 2 == 0 { 2 } else { 3 };
    }
  }

  pub fn write_rate_register(&mut self, val: u8) {
    self.flags_rate_register.set(val);

    if !self.flags_rate_register.irq_enabled() {
      self.irq_pending = false;
    }
  }

  pub fn load_buffer(&mut self, val: u8) {
    self.dma_pending = false;

    if self.sample_length > 0 {
      self.sample_buffer = val;
      self.sample_buffer_empty = false;

      self.sample_address = self.sample_address.wrapping_add(1);

      if self.sample_address == 0 {
        self.sample_address = 0x8000;
      }
      self.sample_length -= 1;

      if self.sample_length == 0 {
        if self.flags_rate_register.loop_flag() {
          self.sample_length = self.sample_length_load;
          self.sample_address = self.sample_address_load;
        } else if self.flags_rate_register.irq_enabled() {
          self.irq_pending = true;
        }
      }
    }
  }

  pub fn check_dma_status(&mut self, cycles: i8) {
    if self.init > 0 {
      self.init -= cycles;
      if self.init <= 0 && self.sample_buffer_empty && self.sample_length > 0 {
        self.dma_pending = true;
      }
    }
  }

  pub fn set_sample_address(&mut self, val: u8) {
    self.sample_address_load = 0xc000 + (val as u16) * 64;
  }

  pub fn set_sample_length(&mut self, val: u8) {
    self.sample_length_load = val * 16 + 1
  }
}