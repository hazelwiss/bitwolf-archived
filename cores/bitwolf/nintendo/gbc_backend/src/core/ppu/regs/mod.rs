pub(crate) mod lcdc;
pub(crate) mod lcds;
pub(crate) mod palette;

pub(in crate::core::ppu) struct Regs {
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
            lcdc: lcdc::LCDC::new(),
            lcds: lcds::LCDS::new(),
            bgp: palette::PaletteRegister::new(),
            obp0: palette::PaletteRegister::new(),
            obp1: palette::PaletteRegister::new(),
        }
    }
}
