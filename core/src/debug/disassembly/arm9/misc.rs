use alloc::string::{String, ToString};
use arm_decode::*;

pub fn undef(_: u32) -> String {
    "undefined".to_string()
}

pub fn unpred(_: u32) -> String {
    "unpredictable".to_string()
}
