use crate::bus::Bus;

impl Bus {
    pub(super) fn write_bootrom_toggle(&mut self, val: u8) {
        self.io.bootrom_toggle = val;
        if val != 0 {
            for i in 0..256 {
                self.rom0[i] = self.rom_256bytes[i];
            }
        }
    }

    pub(super) fn read_bootrom_toggle(&mut self) -> u8 {
        self.io.bootrom_toggle
    }
}
