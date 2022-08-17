use std::fmt::Display;

pub struct Logger {}

impl Logger {
    pub fn fatal(self, msg: impl Display) -> ! {
        panic!("{}", msg)
    }

    pub fn warning(&self, _: impl Display) {
        todo!()
    }

    pub fn info(&self, _: impl Display) {
        todo!()
    }
}
