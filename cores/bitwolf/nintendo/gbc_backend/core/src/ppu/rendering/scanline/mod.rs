mod bg_win;
mod sprites;

use crate::ppu::PPU;

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
        self.regs.ly += 1;
        if self.regs.ly < 144 {
            self.change_mode(Mode::OAMScan);
        } else {
            if self.regs.ly == 144 {
                self.change_mode(Mode::VBlank);
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
        if self.scanline_dot_count == 80 {
            self.change_mode(Mode::Drawing);
        }
    }

    fn drawing(&mut self) {
        if self.scanline_dot_count == 172 {
            self.change_mode(Mode::HBlank);
        } else {
            // Render part of the scanline.
            if self.regs.lcdc.bg_and_window_enable {
                self.draw_bg();
                if self.regs.lcdc.window_enable {
                    self.draw_window();
                }
            }
        }
    }
}
