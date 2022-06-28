pub(crate) mod lcd;

pub(super) mod palette;
pub(super) mod pixel_fetcher;
pub(super) mod scanline;
pub(super) mod shift_register;

impl super::PPU {
    fn vram_access(&self, offset: u16) -> u8 {
        self.vram[offset as usize]
    }
}
