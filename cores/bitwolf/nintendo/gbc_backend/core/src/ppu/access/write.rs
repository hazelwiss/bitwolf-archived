use super::PPUReg;
use crate::{
    bus::{
        address_space::{OAM, VRAM},
        Bus,
    },
    ppu::{
        regs::{lcdc, palette},
        PPU,
    },
};

impl PPU {
    pub(crate) fn write_reg(bus: &mut crate::bus::Bus, reg: PPUReg, val: u8) {
        match reg {
            PPUReg::LY => bus.ppu.regs.ly = val,
            PPUReg::SCX => bus.ppu.regs.scx = val,
            PPUReg::SCY => bus.ppu.regs.scy = val,
            PPUReg::WX => bus.ppu.regs.wx = val,
            PPUReg::WY => bus.ppu.regs.wy = val,
            PPUReg::LYC => bus.ppu.regs.lyc = val,
            PPUReg::LCDC => bus.ppu.regs.lcdc = lcdc::LCDC::from_u8(val, &mut bus.ppu),
            PPUReg::LCDS => bus.ppu.regs.lcds = bus.ppu.regs.lcds.from_u8(val),
            PPUReg::BGP => bus.ppu.regs.bgp = palette::PaletteRegister::from_u8(val),
            PPUReg::OBP0 => bus.ppu.regs.obp0 = palette::PaletteRegister::from_u8(val),
            PPUReg::OBP1 => bus.ppu.regs.obp1 = palette::PaletteRegister::from_u8(val),
            PPUReg::OAMDMA => bus.oam_dma((val as u16) << 8),
        }
    }

    pub(crate) fn write_oam(bus: &mut Bus, offset: OAM, val: u8) {
        bus.ppu.oam[offset.get()] = val;
    }

    pub(crate) fn write_vram(bus: &mut Bus, offset: VRAM, val: u8) {
        bus.ppu.vram[offset.get()] = val;
    }
}
