use crate::core::bus::Bus;

impl Bus {
    pub(super) fn oam_dma(&mut self, adr: u16) {
        if adr > 0xDF00 {
            logger::fatal!("DMA transfer on invalid address {adr:04X}");
        }
        for i in 0..self.ppu.oam.len() {
            self.ppu.oam[i] = self.read(adr + i as u16);
        }
    }
}
