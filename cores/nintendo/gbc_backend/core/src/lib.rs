pub mod bus;
pub mod cpu;
pub mod cycles;
pub mod emu;
pub mod ppu;

mod binder;
mod events;

pub struct Emu {
    cpu: cpu::CPU,
    bus: bus::Bus,
}

impl Emu {
    pub fn new(bootrom: [u8; 256], rom: Vec<u8>) -> Self {
        Self {
            cpu: cpu::CPU::new(),
            bus: bus::Bus::new(bootrom, rom),
        }
    }
}
