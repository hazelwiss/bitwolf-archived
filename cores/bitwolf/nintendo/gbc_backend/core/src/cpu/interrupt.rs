#[repr(u8)]
pub enum InterruptBit {
    VBlank = 0b1,
    LCDStat = 0b10,
    Timer = 0b100,
    Serial = 0b1000,
    Joypad = 0b1_0000,
}

impl InterruptBit {
    pub(crate) fn from_bits(bits: u8) -> (Self, u8) {
        if bits & InterruptBit::VBlank as u8 != 0 {
            // VBlank
            (InterruptBit::VBlank, 0x40)
        } else if bits & InterruptBit::LCDStat as u8 != 0 {
            // LCD Stat
            (InterruptBit::LCDStat, 0x48)
        } else if bits & InterruptBit::Timer as u8 != 0 {
            // Timer
            (InterruptBit::Timer, 0x50)
        } else if bits & InterruptBit::Serial as u8 != 0 {
            // Serial
            (InterruptBit::Serial, 0x58)
        } else if bits & InterruptBit::Joypad as u8 != 0 {
            // Joypad
            (InterruptBit::Joypad, 0x60)
        } else {
            logger::fatal!("An interrupt bit was set and IME enabled but no interrupt was fired.")
        }
    }
}
