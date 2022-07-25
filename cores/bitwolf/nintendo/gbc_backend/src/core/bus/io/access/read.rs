use crate::core::{
    bus::{io::IOReg, Bus},
    ppu::PPU,
};

impl Bus {
    pub(crate) fn read_io(&mut self, reg: IOReg) -> u8 {
        match reg {
            IOReg::IE => self.read_ie(),
            IOReg::IF => self.read_if(),
            IOReg::Serial(reg) => self.read_serial(reg),
            IOReg::Timer(reg) => self.read_timer(reg),
            IOReg::PPUReg(reg) => PPU::read_reg(self, reg),
            IOReg::BootromToggle => self.read_bootrom_toggle(),
            IOReg::Joypad => self.read_joypad(),
            IOReg::APUReg(_) => todo!(),
            IOReg::Invalid(index) => {
                logger::warning!("Read from unknown IO register 0x{index:02X}.");
                0xFF
            }
        }
    }
}
