mod access;
mod serial;
mod timer;

use crate::ppu::regs::PPUReg;
use serial::SerialReg;
use timer::TimerReg;

pub(crate) enum IOReg {
    IE,
    IF,
    Serial(SerialReg),
    Timer(TimerReg),
    PPUReg(PPUReg),
    Invalid(u8),
}

impl IOReg {
    pub fn from_index(index: u8) -> Self {
        match index {
            0xFF => Self::IE,
            0x01 => Self::Serial(SerialReg::SB),
            0x02 => Self::Serial(SerialReg::SC),
            0x04 => Self::Timer(TimerReg::DIV),
            0x05 => Self::Timer(TimerReg::TIMA),
            0x06 => Self::Timer(TimerReg::TMA),
            0x07 => Self::Timer(TimerReg::TAC),
            0x0F => Self::IF,
            0x44 => Self::PPUReg(PPUReg::LY),
            index => Self::Invalid(index),
        }
    }
}

pub(crate) struct IO {
    ie_f: u8,
    if_f: u8,
    serial: serial::Serial,
    timer: timer::Timer,
}

impl IO {
    pub fn new() -> Self {
        Self {
            ie_f: 0,
            if_f: 0,
            serial: serial::Serial::new(),
            timer: timer::Timer::new(),
        }
    }

    pub fn ie_get(&self) -> u8 {
        self.ie_f
    }

    pub fn if_get(&self) -> u8 {
        self.if_f
    }

    pub fn if_set(&mut self, val: u8) {
        self.if_f = val;
    }
}
