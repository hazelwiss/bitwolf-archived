pub mod ptrs;

mod fallback;

use self::ptrs::Ptrs;
use crate::{core::bus::AccessType, Core};

macro_rules! read {
    ($($fn_ident:ident, $ty:ty, $fallback:path;)*) => {
        $(
            pub fn $fn_ident<A: AccessType>(core: &mut Core, adr: u32) -> $ty {
                if let Some(ptr) = core.arm9.bus_ptrs.read(adr) {
                    unsafe {
                        let mask = core::mem::size_of::<$ty>() - 1;
                        let mask = Ptrs::PG_MASK as usize & !mask;
                        ptr.add(adr as usize & mask).cast::<$ty>().read().to_le()
                    }
                } else {
                    $fallback(core, adr)
                }
            }
        )*

    };
}

macro_rules! write {
    ($($fn_ident:ident, $ty:ty, $write_fn:ident, $fallback:path;)*) => {
        $(
            pub fn $fn_ident<A: AccessType>(core: &mut Core, adr: u32, val: $ty) {
                if let Some(ptr) = core.arm9.bus_ptrs.$write_fn(adr) {
                    unsafe {
                        let mask = core::mem::size_of::<$ty>() - 1;
                        let mask = Ptrs::PG_MASK as usize & !mask;
                        let val = val.to_le();
                        ptr.add(adr as usize & mask).cast::<$ty>().write(val)
                    };
                } else {
                    $fallback(core, adr, val);
                }
            }
        )*

    };
}

read! {
    read8, u8, fallback::read8::<A>;
    read16, u16, fallback::read16::<A>;
    read32, u32, fallback::read32::<A>;
}

write! {
    write8, u8, write8, fallback::write8::<A>;
    write16, u16, write32_16, fallback::write16::<A>;
    write32, u32, write32_16, fallback::write32::<A>;
}
