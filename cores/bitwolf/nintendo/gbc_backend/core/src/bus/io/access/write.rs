use crate::bus::{io::IOReg, Bus};

impl Bus {
    pub(crate) fn write_io(&mut self, reg: IOReg, val: u8) {
        match reg {
            IOReg::IE => self.write_ie(val),
            IOReg::IF => self.write_if(val),
            IOReg::Serial(reg) => self.write_serial(reg, val),
            IOReg::Timer(reg) => self.write_timer(reg, val),
            IOReg::PPUReg(reg) => self.write_ppu_reg(reg, val),
            IOReg::BootromToggle => self.write_bootrom_toggle(val),
            IOReg::Joypad => self.write_joypad(val),
            IOReg::APUReg(_) => todo!(),
            IOReg::Invalid(index) => {
                logger::warning!("Write to unknown IO register 0x{index:02X}.");
            }
        }
    }
}
