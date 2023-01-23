use crate::core::{bus::address_space, ppu::PPU};

pub(crate) fn read_vram(ppu: &PPU, adr: address_space::VRAM) -> u8 {
    ppu.vram[adr.get()]
}

pub(crate) fn read_oam(ppu: &PPU, adr: address_space::OAM) -> u8 {
    ppu.oam[adr.get()]
}
