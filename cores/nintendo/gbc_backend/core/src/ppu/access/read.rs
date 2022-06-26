use crate::ppu::{regs::PPUReg, PPU};

impl PPU {
    pub fn read_vram(&self, offset: usize) -> u8 {
        self.vram[offset]
    }

    pub fn read_oam(&self, offset: usize) -> u8 {
        self.oam[offset]
    }

    pub fn read_reg(&mut self, reg: PPUReg) -> u8 {
        self.regs.read(reg)
    }
}
