use super::Interpreter;

impl Interpreter {
    pub(crate) fn check_events(&mut self) {
        if let Some(event) = self.scheduler.dispatch(self.cycle_counter) {
            match event {
                Event::EI => self.event_ei(),
            }
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Event {
    EI,
}

impl Interpreter {
    fn event_ei(&mut self) {
        self.cpu.ime_set(true);
    }
}
