pub enum PPUReg {
    LY,
    Invalid(u8),
}

pub struct Regs {
    ly: u8,
    scx: u8,
    scy: u8,
    wx: u8,
    wy: u8,
    lyc: u8,
}

impl Regs {
    pub fn new() -> Self {
        Self {
            ly: 0,
            scx: 0,
            scy: 0,
            wx: 0,
            wy: 0,
            lyc: 0,
        }
    }

    pub fn read(&mut self, reg: PPUReg) -> u8 {
        match reg {
            PPUReg::LY => 0x90,
            PPUReg::Invalid(_) => todo!(),
        }
    }

    pub fn write(&mut self, reg: PPUReg, val: u8) {
        match reg {
            PPUReg::LY => {}
            PPUReg::Invalid(_) => todo!(),
        }
    }
}
