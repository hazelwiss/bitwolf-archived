use crate::{core::emu::event_slots::Slot, Engine};

use super::CPU;

impl<E: Engine> CPU<E> {
    pub(crate) fn handle_event(&mut self, slot: Slot) {
        match slot {
            Slot::TIMER => self.bus.timer_event(),
            Slot::EI => self.ime_set(true),
        }
    }
}
