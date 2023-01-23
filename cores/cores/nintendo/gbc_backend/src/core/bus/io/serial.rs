use crate::core::bus::Bus;

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
    pub(super) fn write_serial(&mut self, reg: SerialReg, _: u8) {
        match reg {
            SerialReg::SB => {}
            SerialReg::SC => {}
        }
    }

    pub(super) fn read_serial(&mut self, reg: SerialReg) -> u8 {
        match reg {
            SerialReg::SB => todo!(),
            SerialReg::SC => todo!(),
        }
    }
}
