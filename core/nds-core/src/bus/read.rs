use super::{access::BusAccess, Pagable};

pub fn read32<A: BusAccess>(accessed: &mut impl Pagable, adr: u32) -> u32 {
    todo!()
}

pub fn read16<A: BusAccess>(accessed: &mut impl Pagable, adr: u32) -> u16 {
    todo!()
}

pub fn read8<A: BusAccess>(accessed: &mut impl Pagable, adr: u32) -> u8 {
    todo!()
}
