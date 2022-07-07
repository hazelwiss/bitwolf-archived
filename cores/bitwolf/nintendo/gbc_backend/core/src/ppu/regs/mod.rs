pub(crate) mod lcdc;
pub(crate) mod lcds;
pub(crate) mod palette;

pub(crate) struct Regs {
    pub ly: u8,
    pub scx: u8,
    pub scy: u8,
    pub wx: u8,
    pub wy: u8,
    pub lyc: u8,
    pub lcdc: lcdc::LCDC,
    pub lcds: lcds::LCDS,
    pub bgp: palette::PaletteRegister,
    pub obp0: palette::PaletteRegister,
    pub obp1: palette::PaletteRegister,
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
            lcdc: lcdc::LCDC {
                enable: false,
                window_tile_map_area: lcdc::TileMapArea::A9800_9BFF,
                window_enable: false,
                bg_and_window_tile_data_area: lcdc::TileDataArea::A8800_97FF,
                bg_tile_map_area: lcdc::TileMapArea::A9800_9BFF,
                obj_size: lcdc::OBJSize::S8x8,
                obj_enable: false,
                bg_and_window_enable: false,
            },
            lcds: lcds::LCDS {
                lyc_sis: false,
                oam_sis: false,
                vblank_sis: false,
                hblank_sis: false,
                lyc_flag: false,
                mode: super::rendering::scanline::Mode::OAMScan,
            },
            bgp: palette::PaletteRegister::new(),
            obp0: palette::PaletteRegister::new(),
            obp1: palette::PaletteRegister::new(),
        }
    }
}
