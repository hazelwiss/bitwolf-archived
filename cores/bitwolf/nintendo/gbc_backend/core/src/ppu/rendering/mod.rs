pub(crate) mod lcd;

pub(super) mod palette;
pub(super) mod pixel_fetcher;
pub(super) mod scanline;
pub(super) mod shift_register;

use crate::bus::address_space::VRAM;

impl super::PPU {
    fn vram_access(&self, offset: VRAM) -> u8 {
        self.vram[offset.get()]
    }
}
