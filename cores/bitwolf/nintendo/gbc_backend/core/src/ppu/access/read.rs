use super::PPUReg;
use crate::{
    bus::{
        address_space::{OAM, VRAM},
        Bus,
    },
    ppu::PPU,
};

impl PPU {
    pub(crate) fn read_reg(bus: &mut Bus, reg: PPUReg) -> u8 {
        let ppu = &mut bus.ppu;
        match reg {
            PPUReg::LY => ppu.regs.ly,
            PPUReg::SCX => ppu.regs.scx,
            PPUReg::SCY => ppu.regs.scy,
            PPUReg::WX => ppu.regs.wx,
            PPUReg::WY => ppu.regs.wy,
            PPUReg::LYC => ppu.regs.lyc,
            PPUReg::LCDC => ppu.regs.lcdc.as_u8(),
            PPUReg::LCDS => ppu.regs.lcds.as_u8(),
            PPUReg::BGP => ppu.regs.bgp.as_u8(),
            PPUReg::OBP0 => ppu.regs.obp0.as_u8(),
            PPUReg::OBP1 => ppu.regs.obp1.as_u8(),
            PPUReg::OAMDMA => 0xFF,
        }
    }

    pub(crate) fn read_oam(bus: &Bus, offset: OAM) -> u8 {
        bus.ppu.oam[offset.get()]
    }

    pub(crate) fn read_vram(bus: &Bus, offset: VRAM) -> u8 {
        bus.ppu.vram[offset.get()]
    }
}
