use crate::{
    bus::address_space,
    ppu::{regs::PPUReg, PPU},
};

impl PPU {
    pub fn write_vram(&mut self, offset: address_space::VRAM, val: u8) {
        self.vram[offset.get()] = val;
    }

    pub fn write_oam(&mut self, offset: address_space::OAM, val: u8) {
        self.oam[offset.get()] = val;
    }

    pub fn write_reg(&mut self, reg: PPUReg, val: u8) {
        self.regs.write(reg, val);
    }
}
