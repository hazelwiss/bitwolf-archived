use crate::PPU;

impl PPU {
    pub fn read_vram(&self, offset: usize) -> u8 {
        self.vram[offset]
    }

    pub fn read_oam(&self, offset: usize) -> u8 {
        self.oam[offset]
    }
}
