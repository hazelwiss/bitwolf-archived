pub(super) struct FrameState {
    pub frame_ready: bool,
    pub window_ly: u8,
    pub window_fetching: bool,
}

impl FrameState {
    pub fn new() -> Self {
        Self {
            frame_ready: false,
            window_ly: 0,
            window_fetching: false,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new()
    }
}

pub(super) struct ScanlineState {
    pub dot_count: u32,
    pub lyc_interrupt_fired: bool,
    pub lcd_x: u8,
    pub window_drawing: bool,
    pub to_discard_bg_pixels: u8,
}

impl ScanlineState {
    pub fn new() -> Self {
        Self {
            dot_count: 0,
            lyc_interrupt_fired: false,
            lcd_x: 0,
            window_drawing: false,
            to_discard_bg_pixels: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new()
    }
}
