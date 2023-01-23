pub(in crate::core::ppu) struct LCDS {
    pub lyc_sis: bool,
    pub oam_sis: bool,
    pub vblank_sis: bool,
    pub hblank_sis: bool,
    pub lyc_flag: bool,
    pub mode: crate::core::ppu::rendering::scanline::Mode,
}

impl LCDS {
    pub fn new() -> Self {
        LCDS {
            lyc_sis: false,
            oam_sis: false,
            vblank_sis: false,
            hblank_sis: false,
            lyc_flag: false,
            mode: crate::core::ppu::rendering::scanline::Mode::OAMScan,
        }
    }

    pub fn from_u8(&self, val: u8) -> Self {
        let lyc_sis = val & (1 << 6) != 0;
        let oam_sis = val & (1 << 5) != 0;
        let vblank_sis = val & (1 << 4) != 0;
        let hblank_sis = val & (1 << 3) != 0;
        LCDS {
            lyc_sis,
            oam_sis,
            vblank_sis,
            hblank_sis,
            ..*self
        }
    }

    pub fn as_u8(&self) -> u8 {
        let mut res = 0;
        res |= (self.lyc_sis as u8) << 6;
        res |= (self.oam_sis as u8) << 5;
        res |= (self.vblank_sis as u8) << 4;
        res |= (self.hblank_sis as u8) << 3;
        res |= (self.lyc_flag as u8) << 2;
        res |= (self.mode as u8) & 0b11;
        res
    }
}
