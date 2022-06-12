pub mod conf;
pub mod log;

#[macro_export]
macro_rules! info {
        ($($arg:tt)*) => {{
            logger::log::info(logger::log::Sender::Backend, format!($($arg)*));
        }};
    }

#[macro_export]
macro_rules! warning {
        ($($arg:tt)*) => {{
            logger::log::warning(logger::log::Sender::Backend, format!($($arg)*));
        }};
    }

#[macro_export]
macro_rules! fatal {
        ($($arg:tt)*) => {{
            logger::log::fatal(logger::log::Sender::Backend, format!($($arg)*));
            panic!()
        }};
    }
