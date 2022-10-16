use alloc::string::String;
use arm_decode::*;

pub fn transfer(_: u32) -> String {
    format!("transfer")
}

pub fn misc_transfer(_: u32) -> String {
    format!("misc_transfer")
}

pub fn transfer_multiple(_: u32) -> String {
    format!("transfer multiple")
}

pub fn transfer_double(_: u32) -> String {
    format!("transfer double")
}
