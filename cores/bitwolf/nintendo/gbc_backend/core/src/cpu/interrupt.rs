#[repr(u8)]
pub enum InterruptBit {
    VBlank = 0b1,
    LCDStat = 0b10,
    Timer = 0b100,
    Serial = 0b1000,
    Joypad = 0b1_0000,
}

impl InterruptBit {
    pub(crate) fn vec(&self) -> u8 {
        match self {
            InterruptBit::VBlank => 0x40,
            InterruptBit::LCDStat => 0x48,
            InterruptBit::Timer => 0x50,
            InterruptBit::Serial => 0x58,
            InterruptBit::Joypad => 0x60,
        }
    }
}
