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
