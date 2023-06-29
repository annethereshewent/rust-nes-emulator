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
  cycles: u16,
  step: u16,
  mode: FrameCounterMode
}

impl FrameCounter {
  pub fn new() -> Self {
    Self {
      cycles: STEP_CYCLES[0][0],
      step: 0,
      mode: FrameCounterMode::Step4
    }
  }

  pub fn tick(&mut self, cycles: u16) -> u16 {
    if self.cycles > 0 {
      self.cycles -= cycles;
    }
    if self.cycles <= 0 {
      let clock = self.step;
      self.step += 1;

      if self.step == 5 {
        self.step = 0;
      }

      self.cycles = STEP_CYCLES[self.mode as usize][self.step as usize];

      clock
    } else {
      0
    }
  }
}