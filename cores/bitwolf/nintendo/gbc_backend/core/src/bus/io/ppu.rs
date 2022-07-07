use crate::{
    bus::Bus,
    ppu::regs::{lcdc, lcds, palette},
};

pub enum PPUReg {
    LY,
    LYC,
    SCX,
    SCY,
    WX,
    WY,
    LCDC,
    LCDS,
    BGP,
    OBP0,
    OBP1,
    OAMDMA,
}

impl Bus {
    pub(super) fn read_ppu_reg(&mut self, reg: PPUReg) -> u8 {
        match reg {
            PPUReg::LY => self.ppu.regs.ly,
            PPUReg::SCX => self.ppu.regs.scx,
            PPUReg::SCY => self.ppu.regs.scy,
            PPUReg::WX => self.ppu.regs.wx,
            PPUReg::WY => self.ppu.regs.wy,
            PPUReg::LYC => self.ppu.regs.lyc,
            PPUReg::LCDC => {
                let mut res = 0;
                res |= (self.ppu.regs.lcdc.enable as u8) << 7;
                res |= (self.ppu.regs.lcdc.window_tile_map_area as u8) << 6;
                res |= (self.ppu.regs.lcdc.window_enable as u8) << 5;
                res |= (self.ppu.regs.lcdc.bg_and_window_tile_data_area as u8) << 4;
                res |= (self.ppu.regs.lcdc.bg_tile_map_area as u8) << 3;
                res |= (self.ppu.regs.lcdc.obj_size as u8) << 2;
                res |= (self.ppu.regs.lcdc.obj_enable as u8) << 1;
                res |= self.ppu.regs.lcdc.bg_and_window_enable as u8;
                res
            }
            PPUReg::LCDS => {
                let mut res = 0;
                res |= (self.ppu.regs.lcds.lyc_sis as u8) << 6;
                res |= (self.ppu.regs.lcds.oam_sis as u8) << 5;
                res |= (self.ppu.regs.lcds.vblank_sis as u8) << 4;
                res |= (self.ppu.regs.lcds.hblank_sis as u8) << 3;
                res |= (self.ppu.regs.lcds.lyc_flag as u8) << 2;
                res |= (self.ppu.regs.lcds.mode as u8) & 0b11;
                res
            }
            PPUReg::BGP => self.ppu.regs.bgp.as_byte(),
            PPUReg::OBP0 => self.ppu.regs.obp0.as_byte(),
            PPUReg::OBP1 => self.ppu.regs.obp1.as_byte(),
            PPUReg::OAMDMA => 0xFF,
        }
    }

    pub(super) fn write_ppu_reg(&mut self, reg: PPUReg, val: u8) {
        match reg {
            PPUReg::LY => self.ppu.regs.ly = val,
            PPUReg::SCX => self.ppu.regs.scx = val,
            PPUReg::SCY => self.ppu.regs.scy = val,
            PPUReg::WX => self.ppu.regs.wx = val,
            PPUReg::WY => self.ppu.regs.wy = val,
            PPUReg::LYC => self.ppu.regs.lyc = val,
            PPUReg::LCDC => {
                let enable = val & (1 << 7) != 0;
                let window_tile_map_area = if val & (1 << 6) == 0 {
                    lcdc::TileMapArea::A9800_9BFF
                } else {
                    lcdc::TileMapArea::A9C00_9FFF
                };
                let window_enable = val & (1 << 5) != 0;
                let bg_and_window_tile_data_area = if val & (1 << 4) == 0 {
                    lcdc::TileDataArea::A8800_97FF
                } else {
                    lcdc::TileDataArea::A8000_8FFF
                };
                let bg_tile_map_area = if val & (1 << 3) == 0 {
                    lcdc::TileMapArea::A9800_9BFF
                } else {
                    lcdc::TileMapArea::A9C00_9FFF
                };
                let obj_size = if val & (1 << 2) == 0 {
                    lcdc::OBJSize::S8x8
                } else {
                    lcdc::OBJSize::S8x16
                };
                let obj_enable = val & (1 << 1) != 0;
                let bg_and_window_enable = val & 1 != 0;
                self.ppu.regs.lcdc = lcdc::LCDC {
                    enable,
                    window_tile_map_area,
                    window_enable,
                    bg_and_window_tile_data_area,
                    bg_tile_map_area,
                    obj_size,
                    obj_enable,
                    bg_and_window_enable,
                }
            }
            PPUReg::LCDS => {
                let lyc_sis = val & (1 << 6) != 0;
                let oam_sis = val & (1 << 5) != 0;
                let vblank_sis = val & (1 << 4) != 0;
                let hblank_sis = val & (1 << 3) != 0;
                let lyc_flag = self.ppu.regs.lcds.lyc_flag; // read only property.
                let mode = self.ppu.regs.lcds.mode; // read only property.
                self.ppu.regs.lcds = lcds::LCDS {
                    lyc_sis,
                    oam_sis,
                    vblank_sis,
                    hblank_sis,
                    lyc_flag,
                    mode,
                };
            }
            PPUReg::BGP => self.ppu.regs.bgp = palette::PaletteRegister::from_byte(val),
            PPUReg::OBP0 => self.ppu.regs.obp0 = palette::PaletteRegister::from_byte(val),
            PPUReg::OBP1 => self.ppu.regs.obp1 = palette::PaletteRegister::from_byte(val),
            PPUReg::OAMDMA => self.oam_transfer((val as u16) << 8),
        }
    }

    fn oam_transfer(&mut self, adr: u16) {
        if adr > 0xDF00 {
            logger::fatal!("DMA transfer on invalid address {adr:04X}");
        }
        for i in 0..self.ppu.oam.len() {
            self.ppu.oam[i] = self.read(adr + i as u16);
        }
    }
}
