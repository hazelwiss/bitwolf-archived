use crate::bus::{io::IOReg, Bus};

impl Bus {
    pub(crate) fn read_io(&mut self, reg: IOReg) -> u8 {
        match reg {
            IOReg::IE => self.read_ie(),
            IOReg::IF => self.read_if(),
            IOReg::Serial(reg) => self.read_serial(reg),
            IOReg::Timer(reg) => self.read_timer(reg),
            IOReg::PPUReg(reg) => self.read_ppu_reg(reg),
            IOReg::BootromToggle => self.read_bootrom_toggle(),
            IOReg::Joypad => self.read_joypad(),
            IOReg::Invalid(index) => {
                logger::warning!("Read from unknown IO register 0x{index:02X}.");
                0xFF
            }
        }
    }
}
