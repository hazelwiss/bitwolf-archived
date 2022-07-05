use super::Mode;
use crate::{
    bus::address_space::VRAM,
    ppu::{colour::Colour, PPU},
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
        const COLOUR_LUT: [Colour; 4] = [Colour::C0, Colour::C1, Colour::C2, Colour::C3];
        let start = (8 - self.fetcher.current_sprite.x_pos.clamp(0, 8))
            .clamp(self.sprite_sr.len() as u8, 8);
        for c in start..8 {
            let bit = 1 << (7 - c);
            let lo = (self.fetcher.tile_data_lo & bit != 0) as u8;
            let hi = (self.fetcher.tile_data_hi & bit != 0) as u8;
            let index = (lo | (hi << 1)) as usize;
            let colour = COLOUR_LUT[index];
            self.sprite_sr.push(colour);
        }
        self.fetcher.sprite_fetching = false;
        self.fetcher.change_mode(Mode::Index);
    }

    fn sprite_tile_adr(&self) -> u16 {
        let index = self.fetcher.tile_index;
        let offset = (self.regs.ly - self.fetcher.current_sprite.y_pos) * 2;
        let adr = 0x8000 + index as u16 * 16;
        adr + offset as u16
    }
}
