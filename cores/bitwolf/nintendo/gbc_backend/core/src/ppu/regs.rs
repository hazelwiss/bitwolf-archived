pub enum PPUReg {
    LY,
    LYC,
    SCX,
    SCY,
    WX,
    WY,
    LCDC,
    LCDS,
    Invalid(u8),
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub(super) enum TileMapArea {
    A9800_9BFF = 0,
    A9C00_9FFF = 1,
}

impl TileMapArea {
    pub fn get_map_base_adr(&self) -> u16 {
        match self {
            TileMapArea::A9800_9BFF => 0x9800,
            TileMapArea::A9C00_9FFF => 0x9C00,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub(super) enum TileDataArea {
    A8800_97FF = 0,
    A8000_8FFF = 1,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub(super) enum OBJSize {
    S8x8 = 0,
    S8x16 = 1,
}

pub(super) struct LCDC {
    pub(super) enable: bool,
    pub(super) window_tile_map_area: TileMapArea,
    pub(super) window_enable: bool,
    pub(super) bg_and_window_tile_data_area: TileDataArea,
    pub(super) bg_tile_map_area: TileMapArea,
    pub(super) obj_size: OBJSize,
    pub(super) obj_enable: bool,
    pub(super) bg_and_window_enable: bool,
}

pub(super) struct LCDS {
    pub(super) lyc_sis: bool,
    pub(super) oam_sis: bool,
    pub(super) vblank_sis: bool,
    pub(super) hblank_sis: bool,
    pub(super) lyc_flag: bool,
    pub(super) mode: super::rendering::scanline::Mode,
}

pub struct Regs {
    pub(super) ly: u8,
    pub(super) scx: u8,
    pub(super) scy: u8,
    pub(super) wx: u8,
    pub(super) wy: u8,
    pub(super) lyc: u8,
    pub(super) lcdc: LCDC,
    pub(super) lcds: LCDS,
}

impl Regs {
    pub fn new() -> Self {
        Self {
            ly: 0,
            scx: 0,
            scy: 0,
            wx: 0,
            wy: 0,
            lyc: 0,
            lcdc: LCDC {
                enable: false,
                window_tile_map_area: TileMapArea::A9800_9BFF,
                window_enable: false,
                bg_and_window_tile_data_area: TileDataArea::A8800_97FF,
                bg_tile_map_area: TileMapArea::A9800_9BFF,
                obj_size: OBJSize::S8x8,
                obj_enable: false,
                bg_and_window_enable: false,
            },
            lcds: LCDS {
                lyc_sis: false,
                oam_sis: false,
                vblank_sis: false,
                hblank_sis: false,
                lyc_flag: false,
                mode: super::rendering::scanline::Mode::OAMScan,
            },
        }
    }

    pub fn read(&mut self, reg: PPUReg) -> u8 {
        match reg {
            PPUReg::LY => self.ly,
            PPUReg::SCX => self.scx,
            PPUReg::SCY => self.scy,
            PPUReg::WX => self.wx,
            PPUReg::WY => self.wy,
            PPUReg::LYC => self.lyc,
            PPUReg::LCDC => {
                let mut res = 0;
                res |= (self.lcdc.enable as u8) << 7;
                res |= (self.lcdc.window_tile_map_area as u8) << 6;
                res |= (self.lcdc.window_enable as u8) << 5;
                res |= (self.lcdc.bg_and_window_tile_data_area as u8) << 4;
                res |= (self.lcdc.bg_tile_map_area as u8) << 3;
                res |= (self.lcdc.obj_size as u8) << 2;
                res |= (self.lcdc.obj_enable as u8) << 1;
                res |= self.lcdc.bg_and_window_enable as u8;
                res
            }
            PPUReg::LCDS => {
                let mut res = 0;
                res |= (self.lcds.lyc_sis as u8) << 6;
                res |= (self.lcds.oam_sis as u8) << 5;
                res |= (self.lcds.vblank_sis as u8) << 4;
                res |= (self.lcds.hblank_sis as u8) << 3;
                res |= (self.lcds.lyc_flag as u8) << 2;
                res |= (self.lcds.mode as u8) & 0b11;
                res
            }
            PPUReg::Invalid(reg) => logger::fatal!("Read from invalid PPU register '{reg:02X}'"),
        }
    }

    pub fn write(&mut self, reg: PPUReg, val: u8) {
        match reg {
            PPUReg::LY => self.ly = val,
            PPUReg::SCX => self.scx = val,
            PPUReg::SCY => self.scy = val,
            PPUReg::WX => self.wx = val,
            PPUReg::WY => self.wy = val,
            PPUReg::LYC => self.lyc = val,
            PPUReg::LCDC => {
                let enable = val & (1 << 7) != 0;
                let window_tile_map_area = if val & (1 << 6) == 0 {
                    TileMapArea::A9800_9BFF
                } else {
                    TileMapArea::A9C00_9FFF
                };
                let window_enable = val & (1 << 5) != 0;
                let bg_and_window_tile_data_area = if val & (1 << 4) == 0 {
                    TileDataArea::A8800_97FF
                } else {
                    TileDataArea::A8000_8FFF
                };
                let bg_tile_map_area = if val & (1 << 3) == 0 {
                    TileMapArea::A9800_9BFF
                } else {
                    TileMapArea::A9C00_9FFF
                };
                let obj_size = if val & (1 << 2) == 0 {
                    OBJSize::S8x8
                } else {
                    OBJSize::S8x16
                };
                let obj_enable = val & (1 << 1) != 0;
                let bg_and_window_enable = val & 1 != 0;
                self.lcdc = LCDC {
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
                let lyc_flag = self.lcds.lyc_flag; // read only property.
                let mode = self.lcds.mode; // read only property.
                self.lcds = LCDS {
                    lyc_sis,
                    oam_sis,
                    vblank_sis,
                    hblank_sis,
                    lyc_flag,
                    mode,
                };
            }
            PPUReg::Invalid(reg) => logger::fatal!("Write to invalid PPU register '{reg:02X}'"),
        }
    }
}
