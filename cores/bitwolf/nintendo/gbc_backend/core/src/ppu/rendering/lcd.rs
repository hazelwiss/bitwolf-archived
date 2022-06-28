use crate::ppu::PPU;
use common_core::framebuffer::{self, textures::TextureInfo};

use super::palette::Colour;

pub type TextCol = util::colour::BGRA;
pub type Texture = framebuffer::textures::Texture<TextCol, 160, 144>;
pub type FrameBuffer = framebuffer::AccessW<Texture>;

impl PPU {
    pub(super) fn push_to_lcd(&mut self, col: Colour) {
        let y = self.regs.ly as usize;
        let x = self.lcd_x;
        self.lcd_x += 1;
        debug_assert!(
            y < Texture::HEIGHT,
            "Cannot push to lcd with ly values above 144!"
        );
        debug_assert!(
            x < Texture::WIDTH,
            "Cannot fetch pixels with x coordinate of 160 and above!"
        );
        let colour = match col {
            Colour::C0 => TextCol::new(0xFF, 0x00, 0x00, 0xFF),
            Colour::C1 => TextCol::new(0x00, 0xFF, 0x00, 0xFF),
            Colour::C2 => TextCol::new(0x00, 0x00, 0xFF, 0xFF),
            Colour::C3 => TextCol::WHITE,
        };
        self.frame.text[y][x] = colour;
    }
}
