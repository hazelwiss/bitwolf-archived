pub(crate) mod address_space;
pub(crate) mod debug;

mod access;
mod io;
mod memory_map;

use crate::{cpu::interrupt::InterruptBit, emu::event_slots::Slot};
use common_core::schedulers::Scheduler;

pub struct Bus {
    pub(crate) ppu: crate::ppu::PPU,
    scheduler: Scheduler<Slot>,
    cycle_counter: u64,
    rom_256bytes: [u8; 256],
    rom0: [u8; 0x4000],
    rom1: [u8; 0x4000],
    eram: [u8; 0x2000],
    wram0: [u8; 0x1000],
    wram1: [u8; 0x1000],
    io: io::IO,
    hram: [u8; 0x7E],
}

impl Bus {
    pub fn new(bootrom: [u8; 256], rom: Vec<u8>) -> Self {
        if rom.len() > 0x8000 {
            logger::fatal!("ROM too large!");
        }
        if rom.len() < 256 {
            logger::fatal!("ROM too smal!");
        }
        let mut rom0 = [0; 0x4000];
        let mut rom1 = [0; 0x4000];
        let mut rom_256bytes = [0; 256];
        for i in 0..256 {
            rom0[i] = bootrom[i];
            rom_256bytes[i] = rom[i];
        }
        for i in 0x100..0x4000 {
            if i >= rom.len() {
                break;
            }
            rom0[i] = rom[i];
        }
        for i in 0x4000..0x8000 {
            if i >= rom.len() {
                break;
            }
            rom1[i - 0x4000] = rom[i];
        }
        Self {
            ppu: crate::ppu::PPU::new(),
            scheduler: Scheduler::new(),
            cycle_counter: 0,
            rom0,
            rom1,
            rom_256bytes,
            eram: [0; 0x2000],
            wram0: [0; 0x1000],
            wram1: [0; 0x1000],
            io: io::IO::new(),
            hram: [0; 0x7E],
        }
    }

    #[inline(always)]
    pub fn schedule_event(&mut self, ts: u64, s: Slot) {
        self.scheduler.schedule(self.cycle_counter + ts, s)
    }

    #[inline(always)]
    pub fn dispatch_event(&mut self) -> Option<Slot> {
        self.scheduler.dispatch(self.cycle_counter)
    }

    #[inline(always)]
    pub fn unschedule_event(&mut self, s: Slot) {
        self.scheduler.deschedule(s);
    }

    #[inline(always)]
    pub fn tick(&mut self, t_cycles: u64) {
        self.cycle_counter += t_cycles;
        self.ppu.tick(t_cycles as u32);
    }

    #[inline(always)]
    pub fn interrupt_pending(&self) -> Option<InterruptBit> {
        let ie = &self.io.ie;
        if self.ppu.if_vblank && ie.vblank() {
            Some(InterruptBit::VBlank)
        } else if self.ppu.if_stat && ie.stat() {
            Some(InterruptBit::LCDStat)
        } else if self.io.if_timer && ie.timer() {
            Some(InterruptBit::Timer)
        } else if self.io.if_serial && ie.serial() {
            Some(InterruptBit::Serial)
        } else if self.io.if_joypad && ie.joypad() {
            Some(InterruptBit::Joypad)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn if_toggle(&mut self, bit: InterruptBit) {
        match bit {
            InterruptBit::VBlank => self.ppu.if_vblank = !self.ppu.if_vblank,
            InterruptBit::LCDStat => self.ppu.if_stat = !self.ppu.if_stat,
            InterruptBit::Timer => self.io.if_timer = !self.io.if_timer,
            InterruptBit::Serial => self.io.if_serial = !self.io.if_serial,
            InterruptBit::Joypad => self.io.if_joypad = !self.io.if_joypad,
        }
    }
}
