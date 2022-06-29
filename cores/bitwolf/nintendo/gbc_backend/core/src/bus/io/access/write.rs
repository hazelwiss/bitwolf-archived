use crate::bus::{io::IOReg, Bus};

impl Bus {
    pub(crate) fn write_io(&mut self, reg: IOReg, val: u8) {
        match reg {
            IOReg::IE => self.io.ie_f = val,
            IOReg::IF => self.io.if_f = val,
            IOReg::Serial(reg) => self.write_serial(reg, val),
            IOReg::Timer(reg) => self.write_timer(reg, val),
            IOReg::PPUReg(reg) => self.ppu.write_reg(reg, val),
            IOReg::BootromToggle => {
                self.io.bootrom_toggle = val;
                if val != 0 {
                    for i in 0..256 {
                        self.rom0[i] = self.rom_256bytes[i];
                    }
                }
            }
            IOReg::Invalid(index) => {
                logger::warning!("Write to unknown IO register 0x{index:02X}.");
            }
        }
    }
}
