use super::palette::Colour;
use crate::{bus::address_space::VRAM, ppu::PPU};

#[derive(Debug)]
enum Mode {
    Index,
    DataLo,
    DataHi,
    Push,
    Sleep,
}

#[derive(Debug)]
pub struct PixelFetcher {
    pub(super) x: u8,
    fetcherx: u8,
    fetchery: u8,
    tile_index: u8,
    tile_data_lo: u8,
    tile_data_hi: u8,
    mode: Mode,
    mode_dot_progress: u8,
}

impl PixelFetcher {
    pub fn new() -> Self {
        Self {
            x: 0,
            fetcherx: 0,
            fetchery: 0,
            tile_index: 0,
            tile_data_hi: 0,
            tile_data_lo: 0,
            mode: Mode::Index,
            mode_dot_progress: 0,
        }
    }

    fn change_mode(&mut self, mode: Mode) {
        self.mode_dot_progress = 0;
        self.mode = mode;
    }
}

impl PPU {
    pub(super) fn progress_pixel_fetcher(&mut self) {
        let progress = self.pixel_fetcher.mode_dot_progress;
        self.pixel_fetcher.mode_dot_progress += 1;
        match self.pixel_fetcher.mode {
            Mode::Index => self.pixel_fetcher_fetch_tile_adr(progress),
            Mode::DataLo => self.pixel_fetcher_fetch_tile_data_lo(progress),
            Mode::DataHi => self.pixel_fetcher_fetch_tile_data_hi(progress),
            Mode::Sleep => self.pixel_fetcher_sleep(progress),
            Mode::Push => self.pixel_fetcher_push(progress),
        }
    }

    fn pixel_fetcher_fetch_tile_adr(&mut self, progress: u8) {
        if progress == 0 {
            let window = self.pixel_fetcher.x >= self.regs.wx
                && self.regs.ly >= self.regs.wy
                && self.regs.lcdc.window_enable;
            let (map_adr, x, y) = if window {
                todo!();
            } else {
                let map_adr = self.regs.lcdc.bg_tile_map_area.get_map_base_adr();
                let x = self.pixel_fetcher.x + self.regs.scx / 8;
                let y = ((self.regs.ly as u16 + self.regs.scy as u16) / 8) as u8;
                (map_adr, x, y)
            };
            self.pixel_fetcher.fetcherx = x;
            self.pixel_fetcher.fetchery = y;
            let tile_index_adr = map_adr + (x as u16 % 32) + (y as u16 % 32) * 32;
            self.pixel_fetcher.tile_index = self.vram_access(VRAM::new(tile_index_adr));
        } else {
            self.pixel_fetcher.change_mode(Mode::DataLo);
        }
    }

    fn pixel_fetcher_fetch_tile_data_lo(&mut self, progress: u8) {
        if progress == 0 {
            let adr = self.compute_tile_address();
            self.pixel_fetcher.tile_data_lo = self.vram_access(VRAM::new(adr));
        } else {
            self.pixel_fetcher.change_mode(Mode::DataHi);
        }
    }

    fn pixel_fetcher_fetch_tile_data_hi(&mut self, progress: u8) {
        if progress == 0 {
            let adr = self.compute_tile_address();
            self.pixel_fetcher.tile_data_hi = self.vram_access(VRAM::new(adr + 1));
        } else {
            self.pixel_fetcher.change_mode(Mode::Sleep);
        }
    }

    fn pixel_fetcher_sleep(&mut self, progress: u8) {
        if progress >= 1 {
            self.pixel_fetcher.change_mode(Mode::Push);
        }
    }

    fn pixel_fetcher_push(&mut self, _progress: u8) {
        const COLOUR_LUT: [Colour; 4] = [Colour::C0, Colour::C1, Colour::C2, Colour::C3];
        for c in 0..8 {
            let bit = 1 << (7 - c);
            let lo = (self.pixel_fetcher.tile_data_lo & bit != 0) as u8;
            let hi = (self.pixel_fetcher.tile_data_hi & bit != 0) as u8;
            let index = (lo | (hi << 1)) as usize;
            let colour = COLOUR_LUT[index];
            self.bg_win_sr.push(colour);
        }
        self.pixel_fetcher.x += 1;
        self.pixel_fetcher.change_mode(Mode::Index);
    }

    fn compute_tile_address(&self) -> u16 {
        let index = self.pixel_fetcher.tile_index;
        let offset = ((self.regs.scy.wrapping_add(self.regs.ly)) % 8) * 2;
        let adr = match self.regs.lcdc.bg_and_window_tile_data_area {
            crate::ppu::regs::TileDataArea::A8800_97FF => {
                (0x9000 + ((index as i8 as i32) * 16)) as u16
            }
            crate::ppu::regs::TileDataArea::A8000_8FFF => 0x8000 + index as u16 * 16,
        };
        adr + offset as u16
    }
}
