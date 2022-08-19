use super::{access::BusAccess, Pagable};

pub fn write32<A: BusAccess>(accessed: &mut impl Pagable, adr: u32, val: u32) {
    todo!()
}

pub fn write16<A: BusAccess>(accessed: &mut impl Pagable, adr: u32, val: u16) {
    todo!()
}

pub fn write8<A: BusAccess>(accessed: &mut impl Pagable, adr: u32, val: u8) {
    todo!()
}
