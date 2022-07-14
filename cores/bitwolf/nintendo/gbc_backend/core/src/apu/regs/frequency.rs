#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Selection {
    Counter = 0,
    Consecutive = 1,
}

pub(in crate::apu) struct Frequency {
    pub lo_freq: u8,
    pub hi_freq: u8,
    pub selection: Selection,
}

impl Frequency {
    pub fn new() -> Self {
        Self {
            lo_freq: 0,
            hi_freq: 0,
            selection: Selection::Counter,
        }
    }

    pub fn new_lo(&mut self, val: u8) {
        self.lo_freq = val;
    }

    pub fn new_hi(&mut self, val: u8) {
        self.hi_freq = val & 0b111;
        self.selection = if (val >> 6) & 0b1 != 0 {
            Selection::Consecutive
        } else {
            Selection::Counter
        };
    }

    pub fn lo_as_u8(&self) -> u8 {
        0x00
    }

    pub fn hi_as_u8(&self) -> u8 {
        (self.selection as u8) << 6
    }
}
