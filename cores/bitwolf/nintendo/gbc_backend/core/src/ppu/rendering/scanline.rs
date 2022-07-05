use crate::{
    ppu::{
        colour::Colour,
        sprites::{Sprite, SpriteFlags},
        PPU,
    },
    Texture,
};
use common_core::textures::TextureInfo;

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
        self.lyc_check();
        match self.cur_mode {
            Mode::VBlank => {}
            Mode::HBlank => {}
            Mode::OAMScan => self.oam_scan(),
            Mode::Drawing => self.drawing(),
        }
        self.scanline_state.dot_count += 1;
        if self.scanline_state.dot_count >= DOTS_PER_SCANLINE {
            self.on_new_scanline()
        }
    }

    fn window_check(&mut self) {
        if !self.scanline_state.window_drawing && self.regs.lcdc.window_enable {
            let win_x = self.regs.wx - 7;
            let win_y = self.regs.wy;
            self.scanline_state.window_drawing =
                self.fetcher.x >= win_x / 8 && self.regs.ly >= win_y;
        }
    }

    fn lyc_check(&mut self) {
        if self.regs.ly == self.regs.lyc {
            if self.regs.lcds.lyc_sis && !self.scanline_state.lyc_interrupt_fired {
                self.if_stat = true;
                self.scanline_state.lyc_interrupt_fired = true;
            }
            self.regs.lcds.lyc_flag = true;
        } else {
            self.regs.lcds.lyc_flag = false;
        }
    }

    fn on_new_scanline(&mut self) {
        if self.scanline_state.window_drawing {
            self.frame_state.window_ly += 1;
        }
        self.scanline_state.reset();
        self.bg_win_sr.clear();
        self.sprite_sr.clear();
        self.fetcher.clear();
        self.regs.ly += 1;
        if self.regs.ly < 144 {
            self.scanline_state.to_discard_bg_pixels = self.regs.scx % 8;
            self.change_mode(Mode::OAMScan);
        } else {
            if self.regs.ly == 144 {
                self.on_vblank();
            } else if self.regs.ly >= 154 {
                self.on_new_frame();
            }
        }
    }

    fn change_mode(&mut self, new_mode: Mode) {
        match new_mode {
            Mode::HBlank => {
                if self.regs.lcds.hblank_sis {
                    self.if_stat = true;
                }
            }
            Mode::VBlank => {
                if self.regs.lcds.vblank_sis {
                    self.if_stat = true;
                }
            }
            Mode::OAMScan => {
                if self.regs.lcds.oam_sis {
                    self.if_stat = true;
                }
            }
            Mode::Drawing => {}
        }
        self.regs.lcds.mode = new_mode;
        self.cur_mode = new_mode;
    }

    fn oam_scan(&mut self) {
        if self.scanline_state.dot_count >= 80 - 1 {
            self.sprite_buffer.clear();
            for i in 0..40 {
                let index = i * 4;
                let y_pos = self.oam[index] as i8 as i16 - 16;
                let x_pos = self.oam[index + 1] as i8 as i16 - 8;
                let tile_num = self.oam[index + 2];
                let flags = self.oam[index + 3];
                let sprite_height = 8;
                if x_pos > 0
                    && (y_pos <= self.regs.ly as u16 as i16
                        && y_pos + sprite_height > self.regs.ly as u16 as i16)
                {
                    let sprite = Sprite {
                        y_pos: y_pos as u8,
                        x_pos: x_pos as u8,
                        tile_num,
                        flags: SpriteFlags::from_byte(flags),
                    };
                    self.sprite_buffer.push(sprite);
                    if self.sprite_buffer.full() {
                        break;
                    }
                }
            }
            self.change_mode(Mode::Drawing);
        }
    }

    fn on_vblank(&mut self) {
        self.change_mode(Mode::VBlank);
        self.if_vblank = true;
        self.frame_state.frame_ready = true;
    }

    fn on_new_frame(&mut self) {
        self.change_mode(Mode::OAMScan);
        self.frame_state.reset();
        self.regs.ly = 0;
    }

    fn drawing(&mut self) {
        if self.scanline_state.lcd_x < Texture::WIDTH {
            self.window_check();
            self.progress_fetcher();
            self.push_fifo_to_lcd();
        } else {
            self.change_mode(Mode::HBlank);
            self.fetcher.x = 0;
        }
    }

    fn push_fifo_to_lcd(&mut self) {
        if let Some(colour) = self.sr_mix_pixel() {
            if self.scanline_state.to_discard_bg_pixels > 0 {
                self.scanline_state.to_discard_bg_pixels -= 1;
            } else {
                self.push_to_lcd(colour);
            }
        }
    }

    fn sr_mix_pixel(&mut self) -> Option<Colour> {
        if self.bg_win_sr.len() > 0 && !self.fetcher.sprite_fetching {
            let colour = self.bg_win_sr.pop();
            let bg_colour = if self.regs.lcdc.bg_and_window_enable {
                colour
            } else {
                Colour::C0
            };
            let colour = if self.sprite_sr.len() > 0 {
                self.sprite_sr.pop()
            } else {
                bg_colour
            };
            Some(colour)
        } else {
            None
        }
    }
}
