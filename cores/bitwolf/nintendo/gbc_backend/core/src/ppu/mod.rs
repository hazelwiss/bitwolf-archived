pub mod regs;

mod access;
mod rendering;

use crate::FrameBuffer;

pub struct PPU {
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
    regs: regs::Regs,
    cur_mode: rendering::scanline::Mode,
    scanline_dot_count: u32,
    fb: FrameBuffer,
}

impl PPU {
    pub fn new(fb: FrameBuffer) -> Self {
        Self {
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            regs: regs::Regs::new(),
            cur_mode: rendering::scanline::Mode::OAMScan,
            scanline_dot_count: 0,
            fb,
        }
    }
}
