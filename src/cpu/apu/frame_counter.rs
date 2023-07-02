const STEP_CYCLES: [[u16; 6]; 2] = [
  [7457, 7456, 7458, 7457, 1, 1],
  [7457, 7456, 7458, 7458, 7452, 1],
];

#[derive(Copy, Clone)]
pub enum FrameCounterMode {
  Step4,
  Step5
}

pub struct FrameCounter {
  cycles: i16,
  pub step: u16,
  pub mode: FrameCounterMode,
  buffer: Option<u8>,
  write_delay: i8
}

impl FrameCounter {
  pub fn new() -> Self {
    Self {
      cycles: STEP_CYCLES[0][0] as i16,
      step: 0,
      mode: FrameCounterMode::Step4,
      buffer: None,
      write_delay: 0
    }
  }

  pub fn write(&mut self, val: u8, cycles: usize) {
    self.buffer = Some(val);

    self.write_delay = if cycles % 1 == 1 { 4 } else { 3 };
  }

  pub fn poll(&mut self, cycles: u16) -> bool {
    if let Some(val) = self.buffer {
      self.write_delay -= cycles as i8;

      if self.write_delay <= 0 {
        self.update(val, cycles);
        self.buffer = None;

        return true;
      }
    }

    false
  }

  pub fn update(&mut self, val: u8, cycles: u16) {
    if val >> 7 == 1 {
      self.mode = FrameCounterMode::Step5;
    } else {
      self.mode = FrameCounterMode::Step4;
    }

    self.step = 0;
    self.cycles = STEP_CYCLES[self.mode as usize][self.step as usize] as i16;

    if matches!(self.mode, FrameCounterMode::Step5) {
      self.clock(cycles);
    }
  }

  pub fn clock(&mut self, cycles: u16) -> u16 {
    if self.cycles > 0 {
      self.cycles -= cycles as i16;
    }
    if self.cycles <= 0 {
      let clock = self.step;
      self.step += 1;

      if self.step > 5 {
        self.step = 0;
      }

      self.cycles += STEP_CYCLES[self.mode as usize][self.step as usize] as i16;

      clock
    } else {
      0
    }
  }
}