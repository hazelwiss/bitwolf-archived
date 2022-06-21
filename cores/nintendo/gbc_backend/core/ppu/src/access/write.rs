use crate::PPU;

impl PPU {
    pub fn write_vram(&mut self, offset: usize, val: u8) {
        self.vram[offset] = val;
    }

    pub fn write_oam(&mut self, offset: usize, val: u8) {
        self.oam[offset] = val;
    }
}
