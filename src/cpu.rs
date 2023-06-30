pub mod op_codes;
pub mod ppu;
pub mod apu;

use super::cartridge::{Cartridge, Mirroring};
use ppu::PPU;
use apu::APU;

pub struct CPU {
  pub registers: Registers,
  pub memory: [u8; 0x10000],
  pub ppu: PPU,
  pub apu: APU,
  pub prg_length: usize,
  cycles: u16,
  total_cycles: usize,
}

const STACK_BASE_ADDR: u16 = 0x0100;
const STACK_START: u8 = 0xfd;

const NMI_INTERRUPT_VECTOR_ADDRESS: u16 = 0xfffa;
const IRQ_INTERRUPT_VECTOR_ADDRESS: u16 = 0xfffe;

pub struct Registers {
  pub a: u8,
  pub pc: u16,
  pub x: u8,
  pub y: u8,
  pub sp: u8,
  pub p: CpuFlags
}

bitflags! {
  pub struct CpuFlags: u8 {
    const CARRY             = 0b1;
    const ZERO              = 0b10;
    const INTERRUPT_DISABLE = 0b100;
    const DECIMAL_MODE      = 0b1000;
    const BREAK             = 0b10000;
    const BREAK2            = 0b100000;
    const OVERFLOW          = 0b1000000;
    const NEGATIVE          = 0b10000000;
  }
}

impl CPU {
  pub fn new() -> Self {
    CPU {
      registers: Registers {
        a: 0,
        pc: 0,
        p: CpuFlags::from_bits_truncate(0b100100),
        x: 0,
        y: 0,
        sp: STACK_START
      },
      memory: [0; 0x10000],
      prg_length: 0,
      ppu: PPU::new(Vec::new(), Mirroring::Vertical),
      apu: APU::new(),
      cycles: 0,
      total_cycles: 0
    }
  }

  pub fn mem_read(&mut self, address: u16) -> u8 {
    match address {
      0x0000 ..= 0x1fff => self.memory[(address & 0b11111111111) as usize],
      0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => 0,
      0x2002 => self.ppu.read_status_register(),
      0x2004 => self.ppu.read_oam_data(),
      0x2007 => self.ppu.read_data(),
      0x2008 ..=0x3fff => {
        // mirrors
        let mirrored_address = address & 0b00100000_00000111;

        self.mem_read(mirrored_address)
      }
      0x4015 => self.apu.read_status(),
      0x4016 => self.ppu.joypad.read(),
      0x8000 ..= 0xffff => {
        let prg_offset = address - 0x8000;

        if self.prg_length == 0x4000 && prg_offset >= 0x4000 {
          let actual_address = (prg_offset % 0x4000) + 0x8000;

          self.memory[actual_address as usize]
        } else {
          self.memory[address as usize]
        }
      }
      _ => 0
    }
  }

  pub fn mem_write(&mut self, address: u16, value: u8) {
    match address {
      0x0000 ..= 0x1fff => self.memory[(address & 0b11111111111) as usize] = value,
      // 0x2000 ..= 0x3fff => self.memory[(address & 0b100000_00000111) as usize] = value,
      0x2000 => self.ppu.write_to_control(value),
      0x2001 => self.ppu.write_to_mask(value),
      0x2002 => panic!("attempting to write to read only ppu register"),
      0x2003 => self.ppu.write_to_oam_address(value),
      0x2004 => self.ppu.write_to_oam_data(value),
      0x2005 => self.ppu.write_to_scroll(value),
      0x2006 => self.ppu.write_to_ppu_address(value),
      0x2007 => self.ppu.write_to_data(value),
      0x2008 ..=0x3fff => {
        // mirrors
        let mirrored_address = address & 0b00100000_00000111;

        self.mem_write(mirrored_address, value)
      }
      0x4000 => self.apu.pulse1.control.set(value),
      0x4001 => self.apu.pulse1.sweep.set(value),
      0x4002 => self.apu.pulse1.timer_low.set(value),
      0x4003 => self.apu.pulse1.write_timer_high(value),
      0x4004 => self.apu.pulse2.control.set(value),
      0x4005 => self.apu.pulse2.sweep.set(value),
      0x4006 => self.apu.pulse2.timer_low.set(value),
      0x4007 => self.apu.pulse2.write_timer_high(value),
      0x4014 => self.dma_transfer(value),
      0x4015 => self.apu.write_status(value),
      0x4016 => self.ppu.joypad.write(value),
      0x4017 => self.apu.write_frame_counter(value),
      0x8000 ..= 0xffff => panic!("attempting to write to rom"),
      _ => self.ignore_write()
    };
  }

  fn ignore_write(&self) {

  }

  pub fn dma_transfer(&mut self, value: u8) {
    let upper = (value as u16) << 8;

    for i in 0..256 {
      self.ppu.oam_data[self.ppu.oam_address as usize] = self.mem_read(i + upper);
      self.ppu.oam_address = self.ppu.oam_address.wrapping_add(1);
    }
  }

  pub fn mem_write_u16(&mut self, address: u16, value: u16) {
    let lower_byte = (value & 0b11111111) as u8;
    let upper_byte = ((value >> 8) & 0b11111111) as u8;

    self.mem_write(address, lower_byte);
    self.mem_write(address + 1, upper_byte);
  }

  pub fn mem_read_u16(&mut self, address: u16) -> u16 {
    let low_byte = self.mem_read(address) as u16;
    let high_byte = self.mem_read(address + 1) as u16;

    (high_byte << 8) | low_byte
  }

  pub fn load_game(&mut self, cartridge: Cartridge) {
    self.memory[0x8000 .. (0x8000 + cartridge.prg_rom.len())].copy_from_slice(&cartridge.prg_rom[..]);
    self.prg_length = cartridge.prg_rom.len();
    self.ppu.chr_rom = cartridge.chr_rom;
    self.ppu.mirroring = cartridge.mirroring;

    self.registers.pc = self.mem_read_u16(0xfffc);
    // self.registers.pc = 0xc000;
  }

  pub fn tick(&mut self) -> u16 {
    self.cycles = 0;
    if self.ppu.nmi_triggered {
      self.trigger_interrupt(NMI_INTERRUPT_VECTOR_ADDRESS);
      self.ppu.nmi_triggered = false;
    } else if self.apu.irq_pending && !self.registers.p.contains(CpuFlags::INTERRUPT_DISABLE) {
      self.trigger_interrupt(IRQ_INTERRUPT_VECTOR_ADDRESS);
      self.apu.irq_pending = false;
    }

    let op_code = self.mem_read(self.registers.pc);

    self.registers.pc += 1;

    self.decode(op_code);

    self.cycles
  }

  fn trigger_interrupt(&mut self, interrupt_vector_address: u16) {
    let mut flags = self.registers.p.bits().clone();

    self.registers.p.insert(CpuFlags::INTERRUPT_DISABLE);

    // see https://www.nesdev.org/wiki/Status_flags#The_B_flag
    // bit 4 is cleared, bit 5 is set to 1
    flags = flags & !(1 << 4);
    flags = flags | (1 << 5);

    self.push_to_stack_u16(self.registers.pc);
    self.push_to_stack(flags);

    self.cycle(2);
    self.registers.pc = self.mem_read_u16(interrupt_vector_address);
  }

  pub fn push_to_stack(&mut self, val: u8) {
    self.mem_write(STACK_BASE_ADDR + self.registers.sp as u16, val);

    self.registers.sp = self.registers.sp.wrapping_sub(1);
  }

  pub fn push_to_stack_u16(&mut self, val: u16) {
    let lower_byte = (val & 0b11111111) as u8;
    let upper_byte = (val >> 8) as u8;

    self.push_to_stack(upper_byte);
    self.push_to_stack(lower_byte);
  }

  pub fn pop_from_stack(&mut self) -> u8 {
    self.registers.sp = self.registers.sp.wrapping_add(1);

    self.mem_read(STACK_BASE_ADDR + self.registers.sp as u16)
  }

  pub fn pop_from_stack_u16(&mut self) -> u16 {
    let lower_byte = self.pop_from_stack() as u16;
    let upper_byte = self.pop_from_stack() as u16;

    upper_byte << 8 | lower_byte
  }

  pub fn cycle(&mut self, cycles: u16) {
    self.cycles += cycles;
    self.total_cycles += cycles as usize;
    self.ppu.tick(cycles * 3);
    self.apu.tick(cycles);
  }
}