use ppu::{regs::PPUReg, PPU};

pub enum IOReg {
    IE,
    SB,
    SC,
    PPUReg(PPUReg),
    Invalid(u8),
}

impl IOReg {
    pub fn from_index(index: u8) -> Self {
        match index {
            0xFF => Self::IE,
            0x01 => Self::SB,
            0x02 => Self::SC,
            0x44 => Self::PPUReg(PPUReg::LY),
            index => Self::Invalid(index),
        }
    }
}

pub struct IO {
    ie: u8,
}

impl IO {
    pub fn new() -> Self {
        Self { ie: 0 }
    }

    pub fn read(&mut self, ppu: &mut PPU, reg: IOReg) -> u8 {
        match reg {
            IOReg::IE => self.ie,
            IOReg::SB => todo!(),
            IOReg::SC => todo!(),
            IOReg::PPUReg(reg) => ppu.read_reg(reg),
            IOReg::Invalid(index) => {
                logger::warning!("Read from unknown IO register 0x{index:02X}.");
                0xFF
            }
        }
    }

    pub fn write(&mut self, ppu: &mut PPU, reg: IOReg, val: u8) {
        match reg {
            IOReg::IE => self.ie = val,
            IOReg::SB => print!("{}", val as char),
            IOReg::SC => {}
            IOReg::PPUReg(reg) => ppu.write_reg(reg, val),
            IOReg::Invalid(index) => {
                logger::warning!("Write to unknown IO register 0x{index:02X}.");
            }
        }
    }
}
