use super::{Emu, Interpreter};

pub fn read(emu: &Emu<Interpreter>, adr: u16) -> u8 {
    crate::bus::debug::read::read(&emu.bus, adr)
}

pub fn write(_emu: &mut Emu<Interpreter>, _adr: u16, _val: u8) {
    todo!()
}
