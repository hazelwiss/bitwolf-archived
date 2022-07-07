use crate::bus::Bus;
use crate::cpu::interrupt::InterruptBit;

pub struct IE(pub u8);

impl IE {
    pub fn vblank(&self) -> bool {
        self.0 & InterruptBit::VBlank as u8 != 0
    }

    pub fn stat(&self) -> bool {
        self.0 & InterruptBit::LCDStat as u8 != 0
    }

    pub fn timer(&self) -> bool {
        self.0 & InterruptBit::Timer as u8 != 0
    }

    pub fn serial(&self) -> bool {
        self.0 & InterruptBit::Serial as u8 != 0
    }

    pub fn joypad(&self) -> bool {
        self.0 & InterruptBit::Joypad as u8 != 0
    }
}

impl Bus {
    pub(super) fn write_ie(&mut self, val: u8) {
        self.io.ie.0 = val
    }

    pub(super) fn write_if(&mut self, val: u8) {
        let vblank = val & InterruptBit::VBlank as u8 != 0;
        let stat = val & InterruptBit::LCDStat as u8 != 0;
        let timer = val & InterruptBit::Timer as u8 != 0;
        let serial = val & InterruptBit::Serial as u8 != 0;
        let joypad = val & InterruptBit::Joypad as u8 != 0;
        self.ppu.if_vblank = vblank;
        self.ppu.if_stat = stat;
        self.io.if_timer = timer;
        self.io.if_serial = serial;
        self.io.if_joypad = joypad;
    }

    pub(super) fn read_ie(&mut self) -> u8 {
        self.io.ie.0
    }

    pub(super) fn read_if(&mut self) -> u8 {
        self.ppu.if_vblank as u8
            | ((self.ppu.if_stat as u8) << 1)
            | ((self.io.if_timer as u8) << 2)
            | ((self.io.if_serial as u8) << 3)
            | ((self.io.if_joypad as u8) << 4)
    }
}
