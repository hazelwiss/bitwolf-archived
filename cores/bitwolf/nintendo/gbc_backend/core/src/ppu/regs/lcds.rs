pub(in crate::ppu) struct LCDS {
    pub lyc_sis: bool,
    pub oam_sis: bool,
    pub vblank_sis: bool,
    pub hblank_sis: bool,
    pub lyc_flag: bool,
    pub mode: crate::ppu::rendering::scanline::Mode,
}
