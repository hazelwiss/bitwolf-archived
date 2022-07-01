pub(crate) mod debug;
pub(crate) mod regs;

mod access;
mod rendering;

pub(crate) use rendering::lcd;

struct FrameState {
    frame_ready: bool,
    window_ly: u8,
}

impl FrameState {
    fn new() -> Self {
        Self {
            frame_ready: false,
            window_ly: 0,
        }
    }

    fn reset(&mut self) {
        *self = Self::new()
    }
}

struct ScanlineState {
    dot_count: u32,
    lyc_interrupt_fired: bool,
    lcd_x: usize,
    window_drawing: bool,
}

impl ScanlineState {
    fn new() -> Self {
        Self {
            dot_count: 0,
            lyc_interrupt_fired: false,
            lcd_x: 0,
            window_drawing: false,
        }
    }

    fn reset(&mut self) {
        *self = Self::new()
    }
}

pub struct PPU {
    pub(crate) if_stat: bool,
    pub(crate) if_vblank: bool,
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
    regs: regs::Regs,
    bg_win_sr: rendering::shift_register::ShiftRegister,
    sprite_sr: rendering::shift_register::ShiftRegister,
    pixel_fetcher: rendering::pixel_fetcher::PixelFetcher,
    frame: crate::Texture,
    cur_mode: rendering::scanline::Mode,
    frame_state: FrameState,
    scanline_state: ScanlineState,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            if_stat: false,
            if_vblank: false,
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            regs: regs::Regs::new(),
            bg_win_sr: rendering::shift_register::ShiftRegister::new(),
            sprite_sr: rendering::shift_register::ShiftRegister::new(),
            pixel_fetcher: rendering::pixel_fetcher::PixelFetcher::new(),
            frame: crate::Texture::default(),
            cur_mode: rendering::scanline::Mode::OAMScan,
            frame_state: FrameState::new(),
            scanline_state: ScanlineState::new(),
        }
    }

    /// retrieve frame from PPU if frame is ready.
    #[inline(always)]
    pub fn present_frame(&self) -> Option<&crate::Texture> {
        if self.frame_state.frame_ready {
            Some(&self.frame)
        } else {
            None
        }
    }

    /// invalidate ppu frame, preferably after use.
    #[inline(always)]
    pub fn invalidate_frame(&mut self) {
        self.frame_state.frame_ready = false;
    }
}
