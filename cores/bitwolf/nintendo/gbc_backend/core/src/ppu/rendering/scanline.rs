use common_core::framebuffer::textures::TextureInfo;

use crate::{ppu::PPU, Texture};

use super::palette::Colour;

const DOTS_PER_SCANLINE: u32 = 456;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Mode {
    HBlank = 0b00,
    VBlank = 0b01,
    OAMScan = 0b10,
    Drawing = 0b11,
}

impl PPU {
    pub fn tick(&mut self, dots: u32) {
        if self.regs.lcdc.enable {
            for _ in 0..dots {
                self.scanline()
            }
        }
    }

    fn scanline(&mut self) {
        match self.cur_mode {
            Mode::VBlank => {}
            Mode::HBlank => {}
            Mode::OAMScan => self.oam_scan(),
            Mode::Drawing => self.drawing(),
        }
        self.scanline_dot_count += 1;
        if self.scanline_dot_count >= DOTS_PER_SCANLINE {
            self.scanline_done()
        }
    }

    fn scanline_done(&mut self) {
        self.scanline_dot_count = 0;
        self.lcd_x = 0;
        self.regs.ly += 1;
        if self.regs.ly < 144 {
            self.change_mode(Mode::OAMScan);
        } else {
            if self.regs.ly == 144 {
                self.change_mode(Mode::VBlank);
                self.on_vblank();
            } else if self.regs.ly == 154 {
                self.change_mode(Mode::OAMScan);
                self.regs.ly = 0;
            }
        }
    }

    fn change_mode(&mut self, new_mode: Mode) {
        self.regs.lcds.mode = new_mode;
        self.cur_mode = new_mode;
    }

    fn oam_scan(&mut self) {
        if self.scanline_dot_count >= 80 - 1 {
            self.change_mode(Mode::Drawing);
        }
    }

    fn on_vblank(&mut self) {
        self.fb.get().write().text = self.frame.text;
    }

    fn drawing(&mut self) {
        if self.pixel_fetcher.x < Texture::WIDTH as u8 / 8 {
            //println!(
            //    "dot: {}, x: {}, ly: {}, fetcher: {:?}",
            //    self.scanline_dot_count, self.lcd_x, self.regs.ly, self.pixel_fetcher,
            //);
            self.progress_pixel_fetcher();
            if self.bg_win_sr.len() >= 8 {
                for _ in 0..8 {
                    let colour = self.sr_mix_pixel();
                    self.push_to_lcd(colour);
                }
            }
        } else {
            self.change_mode(Mode::HBlank);
            self.pixel_fetcher.x = 0;
        }
    }

    fn sr_mix_pixel(&mut self) -> Colour {
        self.bg_win_sr.pop()
    }
}
