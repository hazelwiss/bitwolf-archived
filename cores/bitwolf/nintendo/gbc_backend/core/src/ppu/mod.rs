pub(crate) mod debug;
pub(crate) mod regs;

mod access;
mod rendering;

pub(crate) use rendering::lcd;

pub struct PPU {
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
    regs: regs::Regs,
    cur_mode: rendering::scanline::Mode,
    scanline_dot_count: u32,
    bg_win_sr: rendering::shift_register::ShiftRegister,
    sprite_sr: rendering::shift_register::ShiftRegister,
    pixel_fetcher: rendering::pixel_fetcher::PixelFetcher,
    frame: crate::Texture,
    frame_ready: bool,
    lcd_x: usize,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            regs: regs::Regs::new(),
            cur_mode: rendering::scanline::Mode::OAMScan,
            scanline_dot_count: 0,
            bg_win_sr: rendering::shift_register::ShiftRegister::new(),
            sprite_sr: rendering::shift_register::ShiftRegister::new(),
            pixel_fetcher: rendering::pixel_fetcher::PixelFetcher::new(),
            frame: crate::Texture::default(),
            frame_ready: false,
            lcd_x: 0,
        }
    }

    /// retrieve frame from PPU if frame is ready.
    #[inline(always)]
    pub fn present_frame(&self) -> Option<&crate::Texture> {
        if self.frame_ready {
            Some(&self.frame)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn invalidate_frame(&mut self) {
        self.frame_ready = false;
    }
}
