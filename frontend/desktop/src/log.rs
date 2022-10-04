use util::log;

struct Drain;

impl Drain {
    #[inline]
    fn linfo(&self, record: &log::Record) {
        println!("[INFO] {}", record.msg())
    }

    #[inline]
    fn lwarn(&self, record: &log::Record) {
        print!("[WARN] {}", record.msg())
    }

    #[inline]
    fn lerr(&self, record: &log::Record) {
        println!("[ERR] {}", record.msg())
    }
}

impl log::Drain for Drain {
    type Ok = ();
    type Err = log::Never;

    fn log(
        &self,
        record: &log::Record,
        _values: &log::OwnedKVList,
    ) -> std::result::Result<Self::Ok, Self::Err> {
        match record.level() {
            log::Level::Error => self.lerr(record),
            log::Level::Warning => self.lwarn(record),
            log::Level::Info => self.linfo(record),
            log::Level::Critical => panic!("critical error"),
            log::Level::Debug => todo!(),
            log::Level::Trace => todo!(),
        }
        Ok(())
    }
}

pub fn new() -> log::Logger {
    util::log::Logger::root(Drain, log::o!())
}
