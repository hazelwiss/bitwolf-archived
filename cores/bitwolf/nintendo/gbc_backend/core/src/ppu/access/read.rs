use crate::{
    bus::address_space,
    ppu::{regs::PPUReg, PPU},
};

impl PPU {
    pub fn read_vram(&self, offset: address_space::VRAM) -> u8 {
        self.vram[offset.get()]
    }

    pub fn read_oam(&self, offset: address_space::OAM) -> u8 {
        self.oam[offset.get()]
    }

    pub fn read_reg(&mut self, reg: PPUReg) -> u8 {
        self.regs.read(reg)
    }
}
