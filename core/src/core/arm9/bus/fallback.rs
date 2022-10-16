use crate::{core::bus::AccessType, Core};

pub fn read8<A: AccessType>(core: &mut Core, adr: u32) -> u8 {
    if A::CPU {
        panic!("fallback {adr:08X}");
    }
    u8::MAX
}

pub fn read16<A: AccessType>(core: &mut Core, adr: u32) -> u16 {
    if A::CPU {
        panic!("fallback {adr:08X}");
    }
    u16::MAX
}

pub fn read32<A: AccessType>(core: &mut Core, adr: u32) -> u32 {
    if A::CPU {
        panic!("fallback {adr:08X}");
    }
    u32::MAX
}

pub fn write8<A: AccessType>(core: &mut Core, adr: u32, val: u8) {
    if A::CPU {
        panic!("fallback {adr:08X}");
    }
}

pub fn write16<A: AccessType>(core: &mut Core, adr: u32, val: u16) {
    if A::CPU {
        panic!("fallback {adr:08X}");
    }
}

pub fn write32<A: AccessType>(core: &mut Core, adr: u32, val: u32) {
    if A::CPU {
        panic!("fallback {adr:08X}");
    }
}
