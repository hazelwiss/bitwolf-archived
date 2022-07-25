use crate::{core::emu::event_slots::Slot, engines::Engine, Emu};

impl<E: Engine> Emu<E> {
    pub(crate) fn handle_event(&mut self, slot: Slot) {
        match slot {
            Slot::TIMER => self.timer_event(),
            Slot::EI => self.cpu.ime_set(true),
        }
    }
}
