pub enum PPUReg {
    LY,
    Invalid(u8),
}

pub struct Regs {}

impl Regs {
    pub fn new() -> Self {
        Self {}
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
