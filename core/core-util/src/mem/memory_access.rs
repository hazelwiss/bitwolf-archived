macro_rules! impl_ma {
    ($($t:ty)*) => {
        $(
            impl MemoryAccess for $t {}
        )*
    };
}

pub trait MemoryAccess {}

impl_ma!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize);
