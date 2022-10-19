use alloc::string::{String, ToString};

pub fn undef(_: u32) -> String {
    "undefined".to_string()
}

pub fn unpred(_: u32) -> String {
    "unpredictable".to_string()
}

pub fn bkpt(_: u32) -> String {
    format!("bkpt")
}

pub fn swi(_: u32) -> String {
    "swi".to_string()
}
