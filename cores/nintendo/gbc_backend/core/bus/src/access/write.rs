#![allow(unused)]
use crate::{
    memory_map::{IOReg, Section},
    Bus,
};

impl Bus {
    pub fn write(&mut self, adr: u16, val: u8) {
        let section = Section::from_adr(adr);
    }
}
