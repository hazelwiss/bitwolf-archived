use crate::Core;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

pub fn disassemble_arm9(core: &Core, adr: u32) -> (String, Vec<u8>) {
    ("".to_string(), vec![])
}

pub fn disassemble_arm7() {
    todo!()
}
