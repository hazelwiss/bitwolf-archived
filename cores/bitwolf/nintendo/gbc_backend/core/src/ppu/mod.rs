pub mod regs;

mod access;
mod rendering;

pub(crate) use rendering::lcd;

use lcd::FrameBuffer;

pub struct PPU {
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
    regs: regs::Regs,
    cur_mode: rendering::scanline::Mode,
    scanline_dot_count: u32,
    bg_win_sr: rendering::shift_register::ShiftRegister,
    sprite_sr: rendering::shift_register::ShiftRegister,
    pixel_fetcher: rendering::pixel_fetcher::PixelFetcher,
    fb: FrameBuffer,
    frame: crate::Texture,
    lcd_x: usize,
}

impl PPU {
    pub fn new(fb: FrameBuffer) -> Self {
        Self {
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            regs: regs::Regs::new(),
            cur_mode: rendering::scanline::Mode::OAMScan,
            scanline_dot_count: 0,
            bg_win_sr: rendering::shift_register::ShiftRegister::new(),
            sprite_sr: rendering::shift_register::ShiftRegister::new(),
            pixel_fetcher: rendering::pixel_fetcher::PixelFetcher::new(),
            fb,
            frame: crate::Texture::default(),
            lcd_x: 0,
        }
    }
}
