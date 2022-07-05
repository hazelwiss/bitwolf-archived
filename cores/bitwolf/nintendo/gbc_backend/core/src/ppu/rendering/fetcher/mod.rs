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
    pub(super) x: u8,
    pub(super) sprite_fetching: bool,
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
            x: 0,
            sprite_fetching: false,
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
        self.x += 1;
        self.change_mode(Mode::Index);
    }

    fn change_mode(&mut self, mode: Mode) {
        self.mode_dot_progress = 0;
        self.mode = mode;
    }
}

impl PPU {
    pub(super) fn progress_fetcher(&mut self) {
        let progress = self.fetcher.mode_dot_progress;
        self.fetcher.mode_dot_progress += 1;
        self.detect_sprite();
        if self.fetcher.sprite_fetching {
            self.progress_sprite_fetcher(progress);
        } else {
            self.progress_pixel_fetcher(progress);
        }
    }

    fn detect_sprite(&mut self) {
        if let Some(sprite) = self.sprite_buffer.pop(self.fetcher.x * 8 + 8) {
            self.fetcher.current_sprite = sprite;
            self.fetcher.sprite_fetching = true;
        }
    }
}
