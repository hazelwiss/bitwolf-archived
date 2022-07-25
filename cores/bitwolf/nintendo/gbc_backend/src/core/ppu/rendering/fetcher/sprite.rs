use super::Mode;
use crate::core::{
    bus::address_space::VRAM,
    ppu::{palette::Index, shift_register::Pixel, PPU},
};

impl PPU {
    pub(super) fn progress_sprite_fetcher(&mut self, progress: u8) {
        match self.fetcher.mode {
            Mode::Index => self.sprite_fetcher_fetch_tile_index(progress),
            Mode::DataLo => self.sprite_fetcher_fetch_tile_data_lo(progress),
            Mode::DataHi => self.sprite_fetcher_fetch_tile_data_hi(progress),
            Mode::Push => self.sprite_fetcher_push(progress),
            Mode::Sleep => logger::fatal!("the sprite fetcher does not sleep"),
        }
    }

    fn sprite_fetcher_fetch_tile_index(&mut self, progress: u8) {
        if progress == 0 {
            self.fetcher.tile_index = self.fetcher.current_sprite.tile_num;
        } else {
            self.fetcher.change_mode(Mode::DataLo);
        }
    }

    fn sprite_fetcher_fetch_tile_data_lo(&mut self, progress: u8) {
        if progress == 0 {
            let adr = self.sprite_tile_adr();
            self.fetcher.tile_data_lo = self.vram_access(VRAM::new(adr));
        } else {
            self.fetcher.change_mode(Mode::DataHi);
        }
    }

    fn sprite_fetcher_fetch_tile_data_hi(&mut self, progress: u8) {
        if progress == 0 {
            let adr = self.sprite_tile_adr();
            self.fetcher.tile_data_hi = self.vram_access(VRAM::new(adr + 1));
        } else {
            self.fetcher.change_mode(Mode::Push);
        }
    }

    fn sprite_fetcher_push(&mut self, _progress: u8) {
        const INDEX_LUT: [Index; 4] = [Index::I0, Index::I1, Index::I2, Index::I3];
        let shifted = self.fetcher.current_sprite.x_pos.clamp(0, 8);
        let x_flip = self.fetcher.current_sprite.flags.x_flip;
        for c in self.sprite_sr.len()..shifted as usize {
            let bit = if x_flip { 1 << c } else { 1 << (7 - c) };
            let lo = (self.fetcher.tile_data_lo & bit != 0) as u8;
            let hi = (self.fetcher.tile_data_hi & bit != 0) as u8;
            let index = (lo | (hi << 1)) as usize;
            let index = INDEX_LUT[index];
            self.sprite_sr.push(Pixel {
                palette: self.fetcher.current_sprite.flags.palette,
                bg_sprite_priority: self.fetcher.current_sprite.flags.priority,
                index,
            });
        }
        self.fetcher.sprite_fetching = false;
        self.fetcher.reset();
    }

    fn sprite_tile_adr(&self) -> u16 {
        let index = self.fetcher.tile_index;
        use crate::core::ppu::regs::lcdc::OBJSize;
        let (width, index) = match self.regs.lcdc.obj_size {
            OBJSize::S8x8 => (7, index),
            OBJSize::S8x16 => (15, index & !1),
        };
        let offset = if self.fetcher.current_sprite.flags.y_flip {
            let subtract = ((self.regs.ly + 16) - self.fetcher.current_sprite.y_pos) % (width + 1);
            (width - subtract) * 2
        } else {
            (((self.regs.ly + 16) - self.fetcher.current_sprite.y_pos) % (width + 1)) * 2
        };
        let adr = 0x8000 + index as u16 * 16;
        adr + offset as u16
    }
}
