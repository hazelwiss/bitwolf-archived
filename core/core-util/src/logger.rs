pub struct Logger {}

impl Logger {
    pub fn fatal(self, msg: impl ToString) -> ! {
        panic!("{}", msg.to_string())
    }

    pub fn warning(&self, _: impl ToString) {}

    pub fn info(&self, _: impl ToString) {}
}
