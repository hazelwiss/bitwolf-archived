#[derive(Clone, Copy)]
#[repr(u8)]
pub(crate) enum SweepTime {
    Off = 0b000,
    Freq7p8 = 0b001,
    Freq15p6 = 0b010,
    Freq23p4 = 0b011,
    Freq31p3 = 0b100,
    Freq39p1 = 0b101,
    Freq46p9 = 0b110,
    Freq54p7 = 0b111,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub(crate) enum SweepIncrease {
    Addition = 0,
    Subtraction = 1,
}

pub(crate) struct SweepReg {
    pub sweep_time: SweepTime,
    pub sweep_increase: SweepIncrease,
    pub number_of_sweep_shift: u8,
}

impl SweepReg {
    pub fn new() -> Self {
        Self::fromu8(0)
    }

    pub fn fromu8(val: u8) -> Self {
        let sweep_time = match (val >> 4) & 0b111 {
            0b000 => SweepTime::Off,
            0b001 => SweepTime::Freq7p8,
            0b010 => SweepTime::Freq15p6,
            0b011 => SweepTime::Freq23p4,
            0b100 => SweepTime::Freq31p3,
            0b101 => SweepTime::Freq39p1,
            0b110 => SweepTime::Freq46p9,
            0b111 => SweepTime::Freq54p7,
            _ => logger::fatal!("Invalid sweep time index for NR10 IO register."),
        };
        let sweep_increase = if (val >> 3) & 0b1 != 0 {
            SweepIncrease::Subtraction
        } else {
            SweepIncrease::Addition
        };
        Self {
            sweep_time,
            sweep_increase,
            number_of_sweep_shift: val & 0b0111,
        }
    }

    pub fn as_u8(&self) -> u8 {
        let mut val = 0;
        val |= (self.sweep_time as u8) << 4;
        val |= (self.sweep_increase as u8) << 3;
        val |= self.number_of_sweep_shift;
        val
    }
}
