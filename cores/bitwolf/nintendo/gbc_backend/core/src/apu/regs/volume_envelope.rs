#[derive(Clone, Copy)]
#[repr(u8)]
pub(crate) enum EnvelopeDirection {
    Decrease = 0,
    Increase = 1,
}

pub(crate) struct VolumeEnvelopeReg {
    pub initial_volume: u8,
    pub envelope_direction: EnvelopeDirection,
    pub number_of_sweep: u8,
}

impl VolumeEnvelopeReg {
    pub fn new() -> Self {
        Self::from_u8(0)
    }

    pub fn from_u8(val: u8) -> Self {
        let initial_volume = val >> 4;
        let envelope_direction = if (val >> 3) & 0b1 != 0 {
            EnvelopeDirection::Increase
        } else {
            EnvelopeDirection::Decrease
        };
        let number_of_sweep = val & 0b111;
        Self {
            initial_volume,
            envelope_direction,
            number_of_sweep,
        }
    }

    pub fn as_u8(&self) -> u8 {
        let mut val = 0;
        val |= self.initial_volume << 4;
        val |= (self.envelope_direction as u8) << 3;
        val |= self.number_of_sweep;
        val
    }
}
