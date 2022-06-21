use ppu::PPU;

pub enum IOReg {
    IE,
    Invalid(u8),
}

impl IOReg {
    pub fn from_index(index: u8) -> Self {
        match index {
            0xFF => Self::IE,
            index => Self::Invalid(index),
        }
    }
}

pub struct IO {}

impl IO {
    pub fn new() -> Self {
        todo!()
    }

    pub fn read(&mut self, ppu: &mut PPU, reg: IOReg) -> u8 {
        todo!()
    }

    pub fn write(&mut self, ppu: &mut PPU, reg: IOReg, val: u8) {
        todo!()
    }
}
