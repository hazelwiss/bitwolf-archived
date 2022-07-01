use crate::{
    bus::{io::IOReg, Bus},
    cpu::interrupt::InterruptBit,
};

impl Bus {
    pub(crate) fn write_io(&mut self, reg: IOReg, val: u8) {
        match reg {
            IOReg::IE => self.io.ie.0 = val,
            IOReg::IF => {
                let vblank = val & InterruptBit::VBlank as u8 != 0;
                let stat = val & InterruptBit::LCDStat as u8 != 0;
                let timer = val & InterruptBit::Timer as u8 != 0;
                let serial = val & InterruptBit::Serial as u8 != 0;
                let joypad = val & InterruptBit::Joypad as u8 != 0;
                self.ppu.if_vblank = vblank;
                self.ppu.if_stat = stat;
                self.io.if_timer = timer;
                self.io.if_serial = serial;
                self.io.if_joypad = joypad;
            }
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
