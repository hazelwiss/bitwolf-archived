pub(super) mod fetcher;
pub(crate) mod lcd;
pub(super) mod scanline;

use crate::core::bus::address_space::VRAM;

impl super::PPU {
    fn vram_access(&self, offset: VRAM) -> u8 {
        self.vram[offset.get()]
    }
}
