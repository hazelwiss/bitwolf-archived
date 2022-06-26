use crate::ppu::{regs::PPUReg, PPU};

impl PPU {
    pub fn write_vram(&mut self, offset: usize, val: u8) {
        self.vram[offset] = val;
    }

    pub fn write_oam(&mut self, offset: usize, val: u8) {
        self.oam[offset] = val;
    }

    pub fn write_reg(&mut self, reg: PPUReg, val: u8) {
        self.regs.write(reg, val);
    }
}
