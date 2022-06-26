use crate::bus::{io::IOReg, Bus};

impl Bus {
    pub(crate) fn write_io(&mut self, reg: IOReg, val: u8) {
        match reg {
            IOReg::IE => self.io.ie_f = val,
            IOReg::IF => self.io.if_f = val,
            IOReg::Serial(reg) => self.write_serial(reg, val),
            IOReg::Timer(reg) => self.write_timer(reg, val),
            IOReg::PPUReg(reg) => self.ppu.write_reg(reg, val),
            IOReg::Invalid(index) => {
                logger::warning!("Write to unknown IO register 0x{index:02X}.");
            }
        }
    }
}
