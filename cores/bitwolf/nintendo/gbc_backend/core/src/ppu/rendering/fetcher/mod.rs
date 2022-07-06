mod pixel;
mod sprite;

use crate::ppu::{sprites::Sprite, PPU};

#[derive(Debug)]
enum Mode {
    Index,
    DataLo,
    DataHi,
    Push,
    Sleep,
}

#[derive(Debug)]
pub struct Fetcher {
    pub(super) sprite_fetching: bool,
    x: u8,
    tile_index: u8,
    tile_data_lo: u8,
    tile_data_hi: u8,
    mode: Mode,
    mode_dot_progress: u8,
    current_sprite: Sprite,
}

impl Fetcher {
    pub fn new() -> Self {
        Self {
            sprite_fetching: false,
            x: 0,
            tile_index: 0,
            tile_data_hi: 0,
            tile_data_lo: 0,
            mode: Mode::Index,
            mode_dot_progress: 0,
            current_sprite: Sprite::none(),
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    fn reset(&mut self) {
        self.change_mode(Mode::Index);
    }

    fn change_mode(&mut self, mode: Mode) {
        self.mode_dot_progress = 0;
        self.mode = mode;
    }
}

impl PPU {
    pub(super) fn progress_fetcher(&mut self) {
        self.detect_sprite();
        let progress = self.fetcher.mode_dot_progress;
        self.fetcher.mode_dot_progress += 1;
        if self.fetcher.sprite_fetching {
            self.progress_sprite_fetcher(progress);
        } else {
            self.progress_pixel_fetcher(progress);
        }
    }

    fn detect_sprite(&mut self) {
        if self.fetcher.sprite_fetching {
            return;
        }
        if let Some(sprite) = self
            .sprite_buffer
            .pop((self.scanline_state.lcd_x + 8) as u8)
        {
            self.fetcher.reset();
            self.fetcher.current_sprite = sprite;
            self.fetcher.sprite_fetching = true;
        }
    }
}
