#[derive(Clone, Copy)]
#[repr(u8)]
pub(crate) enum WavePattern {
    Percent12p5 = 0b00,
    Percent25 = 0b01,
    Percent50 = 0b10,
    Percent75 = 0b11,
}

pub(crate) struct PatternDutyReg {
    pub wave_pattern: WavePattern,
    pub sound_length: u8,
}

impl PatternDutyReg {
    pub fn new() -> Self {
        Self::from_u8(0)
    }

    pub fn from_u8(val: u8) -> Self {
        let wave_pattern = match (val >> 6) & 0b11 {
            0b00 => WavePattern::Percent12p5,
            0b01 => WavePattern::Percent25,
            0b10 => WavePattern::Percent50,
            0b11 => WavePattern::Percent75,
            _ => logger::fatal!("Invalid wave pattern index for NR11 IO register."),
        };
        let sound_length = val & 0b0011_1111;
        Self {
            wave_pattern,
            sound_length,
        }
    }

    pub fn as_u8(&self) -> u8 {
        let mut val = 0;
        val |= (self.wave_pattern as u8) << 6;
        val |= self.sound_length;
        val
    }
}
