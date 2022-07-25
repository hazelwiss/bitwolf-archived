use super::Mode;
use crate::core::{
    bus::address_space::VRAM,
    ppu::{
        palette::Index, palette::Palette, regs::lcdc::TileDataArea, shift_register::Pixel,
        sprites::SpritePriority, PPU,
    },
};

impl PPU {
    pub(super) fn progress_pixel_fetcher(&mut self, progress: u8) {
        self.window_check();
        match self.fetcher.mode {
            Mode::Index => self.pixel_fetcher_fetch_tile_index(progress),
            Mode::DataLo => self.pixel_fetcher_fetch_tile_data_lo(progress),
            Mode::DataHi => self.pixel_fetcher_fetch_tile_data_hi(progress),
            Mode::Sleep => self.pixel_fetcher_sleep(progress),
            Mode::Push => self.pixel_fetcher_push(progress),
        }
    }

    fn pixel_fetcher_fetch_tile_index(&mut self, progress: u8) {
        if progress == 0 {
            let window = self.scanline_state.window_drawing;
            let (map_adr, x, y) = if window {
                let map_adr = self.regs.lcdc.window_tile_map_area.get_map_base_adr();
                let win_x = self.regs.wx - 7;
                let x = self.fetcher.x - win_x / 8;
                let y = self.frame_state.window_ly / 8;
                (map_adr, x, y)
            } else {
                let map_adr = self.regs.lcdc.bg_tile_map_area.get_map_base_adr();
                let x = self.fetcher.x + self.regs.scx / 8;
                let y = ((self.regs.ly as u16 + self.regs.scy as u16) / 8) as u8;
                (map_adr, x, y)
            };
            let tile_index_adr = map_adr + (x as u16 % 32) + (y as u16 % 32) * 32;
            self.fetcher.tile_index = self.vram_access(VRAM::new(tile_index_adr));
        } else {
            self.fetcher.change_mode(Mode::DataLo);
        }
    }

    fn pixel_fetcher_fetch_tile_data_lo(&mut self, progress: u8) {
        if progress == 0 {
            let adr = self.compute_tile_address();
            self.fetcher.tile_data_lo = self.vram_access(VRAM::new(adr));
        } else {
            self.fetcher.change_mode(Mode::DataHi);
        }
    }

    fn pixel_fetcher_fetch_tile_data_hi(&mut self, progress: u8) {
        if progress == 0 {
            let adr = self.compute_tile_address();
            self.fetcher.tile_data_hi = self.vram_access(VRAM::new(adr + 1));
        } else {
            self.fetcher.change_mode(Mode::Sleep);
        }
    }

    fn pixel_fetcher_sleep(&mut self, progress: u8) {
        if progress >= 1 {
            self.fetcher.change_mode(Mode::Push);
        }
    }

    fn pixel_fetcher_push(&mut self, _progress: u8) {
        const INDEX_LUT: [Index; 4] = [Index::I0, Index::I1, Index::I2, Index::I3];
        if self.bg_win_sr.len() == 0 {
            for c in 0..8 {
                let bit = 1 << (7 - c);
                let lo = (self.fetcher.tile_data_lo & bit != 0) as u8;
                let hi = (self.fetcher.tile_data_hi & bit != 0) as u8;
                let index = (lo | (hi << 1)) as usize;
                let index = INDEX_LUT[index];
                self.bg_win_sr.push(Pixel {
                    bg_sprite_priority: SpritePriority::SpritePriority,
                    palette: Palette::OBP0,
                    index,
                });
            }
            self.fetcher.x += 1;
            self.fetcher.reset();
        }
    }

    fn window_check(&mut self) {
        if !self.scanline_state.window_drawing && self.regs.lcdc.window_enable {
            let win_x = (self.regs.wx as i16) - 7;
            let win_y = self.regs.wy;
            let window_drawing = self.fetcher.x as i16 >= win_x / 8
                && (self.regs.ly >= win_y || self.frame_state.window_fetching);
            if window_drawing {
                self.scanline_state.window_drawing = true;
                self.frame_state.window_fetching = true;
            }
        }
    }

    fn compute_tile_address(&self) -> u16 {
        let index = self.fetcher.tile_index;
        let offset = if self.scanline_state.window_drawing {
            (self.frame_state.window_ly % 8) * 2
        } else {
            ((self.regs.scy.wrapping_add(self.regs.ly)) % 8) * 2
        };
        let adr = match self.regs.lcdc.bg_and_window_tile_data_area {
            TileDataArea::A8800_97FF => (0x9000 + ((index as i8 as i32) * 16)) as u16,
            TileDataArea::A8000_8FFF => 0x8000 + index as u16 * 16,
        };
        adr + offset as u16
    }
}
