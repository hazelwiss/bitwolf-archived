use crate::{bus::Bus, cpu::interrupt::InterruptBit, emu::event_slots::Slot, Emu, Engine};

pub(crate) enum TimerReg {
    DIV,
    TIMA,
    TMA,
    TAC,
}

pub(crate) struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }
}

impl Bus {
    pub(crate) fn write_timer(&mut self, reg: TimerReg, val: u8) {
        match reg {
            TimerReg::DIV => self.io.timer.div = 0,
            TimerReg::TIMA => {
                self.io.timer.tima = val;
                self.recalculate_timer_event()
            }
            TimerReg::TMA => self.io.timer.tma = val,
            TimerReg::TAC => {
                self.io.timer.tac = val;
                self.recalculate_timer_event();
            }
        }
    }

    pub(crate) fn read_timer(&mut self, reg: TimerReg) -> u8 {
        match reg {
            TimerReg::DIV => {
                self.io.timer.div =
                    (self.io.timer.div as u64).wrapping_add(self.cycle_counter / 256) as u8;
                self.io.timer.div
            }
            TimerReg::TIMA => todo!(),
            TimerReg::TMA => todo!(),
            TimerReg::TAC => todo!(),
        }
    }

    fn recalculate_timer_event(&mut self) {
        if self.io.timer.tac & 0b100 != 0 {
            let base = 0x100 - self.io.timer.tima as u64;
            let ts = match self.io.timer.tac & 0b11 {
                0 => base * 1024,
                1 => base * 16,
                2 => base * 64,
                3 => base * 256,
                _ => logger::fatal!(""),
            };
            self.schedule_event(ts, Slot::TIMER);
        } else {
            self.unschedule_event(Slot::TIMER);
        }
    }
}

impl<E: Engine> Emu<E> {
    pub(crate) fn timer_event(&mut self) {
        self.bus.if_toggle(InterruptBit::Timer);
        self.bus.io.timer.tima = self.bus.io.timer.tma;
        self.bus.recalculate_timer_event();
    }
}
