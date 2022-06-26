use crate::bus::Bus;

pub(crate) enum SerialReg {
    SB,
    SC,
}

pub(crate) struct Serial {}

impl Serial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Bus {
    pub(crate) fn write_serial(&mut self, reg: SerialReg, val: u8) {
        match reg {
            SerialReg::SB => print!("{}", val as char),
            SerialReg::SC => {}
        }
    }

    pub(crate) fn read_serial(&mut self, reg: SerialReg) -> u8 {
        match reg {
            SerialReg::SB => todo!(),
            SerialReg::SC => todo!(),
        }
    }
}
