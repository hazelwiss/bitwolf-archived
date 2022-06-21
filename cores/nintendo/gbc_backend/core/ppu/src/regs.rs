pub enum PPUReg {
    Invalid(u8),
}

pub struct Regs {}

impl Regs {
    pub fn new() -> Self {
        todo!()
    }

    pub fn read(&mut self, reg: PPUReg) -> u8 {
        todo!()
    }

    pub fn write(&mut self, reg: PPUReg, val: u8) {
        todo!()
    }
}
