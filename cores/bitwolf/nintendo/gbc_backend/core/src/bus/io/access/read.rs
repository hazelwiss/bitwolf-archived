use crate::bus::{io::IOReg, Bus};

impl Bus {
    pub(crate) fn read_io(&mut self, reg: IOReg) -> u8 {
        match reg {
            IOReg::IE => self.io.ie.0,
            IOReg::IF => {
                self.ppu.if_vblank as u8
                    | ((self.ppu.if_stat as u8) << 1)
                    | ((self.io.if_timer as u8) << 2)
                    | ((self.io.if_serial as u8) << 3)
                    | ((self.io.if_joypad as u8) << 4)
            }
            IOReg::Serial(reg) => self.read_serial(reg),
            IOReg::Timer(reg) => self.read_timer(reg),
            IOReg::PPUReg(reg) => self.ppu.read_reg(reg),
            IOReg::BootromToggle => self.io.bootrom_toggle,
            IOReg::Invalid(index) => {
                logger::warning!("Read from unknown IO register 0x{index:02X}.");
                0xFF
            }
        }
    }
}
