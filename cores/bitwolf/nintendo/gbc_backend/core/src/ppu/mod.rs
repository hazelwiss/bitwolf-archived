pub(crate) mod debug;
pub(crate) mod regs;

mod palette;
mod rendering;
mod shift_register;
mod sprites;
mod states;

pub(crate) use rendering::lcd;

pub(crate) struct PPU {
    pub(crate) if_stat: bool,
    pub(crate) if_vblank: bool,
    pub(crate) regs: regs::Regs,
    pub(crate) vram: [u8; 0x2000],
    pub(crate) oam: [u8; 0xA0],
    bg_win_sr: shift_register::ShiftRegister,
    sprite_sr: shift_register::ShiftRegister,
    fetcher: rendering::fetcher::Fetcher,
    sprite_buffer: sprites::SpriteBuffer,
    frame: crate::Texture,
    cur_mode: rendering::scanline::Mode,
    frame_state: states::FrameState,
    scanline_state: states::ScanlineState,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            if_stat: false,
            if_vblank: false,
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            regs: regs::Regs::new(),
            bg_win_sr: shift_register::ShiftRegister::new(),
            sprite_sr: shift_register::ShiftRegister::new(),
            fetcher: rendering::fetcher::Fetcher::new(),
            sprite_buffer: sprites::SpriteBuffer::new(),
            frame: crate::Texture::default(),
            cur_mode: rendering::scanline::Mode::OAMScan,
            frame_state: states::FrameState::new(),
            scanline_state: states::ScanlineState::new(),
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
