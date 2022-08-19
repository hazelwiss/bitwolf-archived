#![feature(allocator_api)]

pub mod mem;

#[macro_export]
macro_rules! kb {
    ($lit:literal) => {{
        (($lit) << 10)
    }};
}

#[macro_export]
macro_rules! mb {
    ($lit:literal) => {{
        (($lit) << 20)
    }};
}

#[macro_export]
macro_rules! gb {
    ($lit:literal) => {{
        (($lit) << 30)
    }};
}
