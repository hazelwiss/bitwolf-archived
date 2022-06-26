use crate::bus::{io::IOReg, Bus};

impl Bus {
    pub(crate) fn read_io(&mut self, reg: IOReg) -> u8 {
        match reg {
            IOReg::IE => self.io.ie_f,
            IOReg::IF => self.io.if_f,
            IOReg::Serial(reg) => self.read_serial(reg),
            IOReg::Timer(reg) => self.read_timer(reg),
            IOReg::PPUReg(reg) => self.ppu.read_reg(reg),
            IOReg::Invalid(index) => {
                logger::warning!("Read from unknown IO register 0x{index:02X}.");
                0xFF
            }
        }
    }
}
