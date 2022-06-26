pub mod regs;

mod access;
mod rendering;

pub struct PPU {
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
    regs: regs::Regs,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            regs: regs::Regs::new(),
        }
    }
}
