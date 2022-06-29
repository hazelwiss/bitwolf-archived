use crate::{bus::address_space, ppu::PPU};

pub fn read_vram(ppu: &PPU, adr: address_space::VRAM) -> u8 {
    ppu.vram[adr.get()]
}

pub fn read_oam(ppu: &PPU, adr: address_space::OAM) -> u8 {
    ppu.oam[adr.get()]
}
