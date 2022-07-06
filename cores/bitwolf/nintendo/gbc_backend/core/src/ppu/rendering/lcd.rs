use crate::ppu::{palette::Colour, PPU};
use common_core::textures::{self, TextureInfo};

pub type TextCol = util::colour::BGRA;
pub type Texture = textures::Texture<TextCol, 160, 144>;

impl PPU {
    pub(super) fn push_to_lcd(&mut self, col: Colour) {
        let y = self.regs.ly as usize;
        let x = self.scanline_state.lcd_x as usize;
        self.scanline_state.lcd_x += 1;
        debug_assert!(
            y < Texture::HEIGHT,
            "Cannot push to lcd with ly values above 144!"
        );
        debug_assert!(
            x < Texture::WIDTH,
            "Cannot fetch pixels with x coordinate of 160 and above!"
        );
        self.frame.data[y][x] = match col {
            Colour::C0 => TextCol::new(0xFF, 0xFF, 0xFF, 0xFF),
            Colour::C1 => TextCol::new(0xCC, 0xCC, 0xCC, 0xFF),
            Colour::C2 => TextCol::new(0x66, 0x66, 0x66, 0xFF),
            Colour::C3 => TextCol::new(0x00, 0x00, 0x00, 0xFF),
        }
    }
}
