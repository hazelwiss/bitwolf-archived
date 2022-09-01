pub use macros::{DumpString, UnsafeDumpString};

pub mod __private {
    pub const PAD_STR: &str = "  ";
}

pub trait DumpString {
    fn dump(&self, depth: usize) -> String;

    fn dump_as_lines(&self, depth: usize) -> Vec<String>;
}

/// # Safety
/// dumps a structure as a string, performing possibly unsafe conversion
/// and pointer access.
pub unsafe trait UnsafeDumpString {
    /// # Safety
    /// `ptr` has to be a valid, possible unaligned, value for `Self`.
    unsafe fn dump(ptr: *const Self, depth: usize) -> String;

    /// # Safety
    /// `ptr` has to be a valid, possible unaligned, value for `Self`.
    unsafe fn dump_as_lines(ptr: *const Self, depth: usize) -> Vec<String>;
}

#[macro_export]
macro_rules! dump {
    ($expr:expr) => {{
        ::util::dumpable::DumpString::dump(&$expr, 0)
    }};
}

#[macro_export]
macro_rules! dump_unsafe {
    ($expr:expr) => {{
        fn ref_as_ptr<T>(var: &T) -> *const T {
            var as *const T
        }
        let ptr = ref_as_ptr(&$expr);
        ::util::dumpable::UnsafeDumpString::dump(ptr, 0)
    }};
}

macro_rules! impl_dump {
    ($($ty:ty: $s:literal),*) => {
        $(
            impl DumpString for $ty {
                fn dump(&self, depth: usize) -> String {
                    let hex_str = format!($s, self);
                    let mut padding = String::new();
                    padding.reserve(depth);
                    for _ in 0..depth{
                        padding.push_str(__private::PAD_STR);
                    }
                    format!("{padding}0x{hex_str} (0b{self:b})")
                }

                fn dump_as_lines(&self, depth: usize) -> Vec<String> {
                    vec![self.dump(depth)]
                }
            }

            unsafe impl UnsafeDumpString for $ty {
                unsafe fn dump(ptr: *const Self, depth: usize) -> String {
                    let hex_str = format!($s, *ptr);
                    let mut padding = String::new();
                    padding.reserve(depth);
                    for _ in 0..depth{
                        padding.push_str(__private::PAD_STR);
                    }
                    format!("{padding}0x{hex_str} (0b{:b})", *ptr)
                }

                unsafe fn dump_as_lines(ptr: *const Self, depth: usize) -> Vec<String> {
                    vec![UnsafeDumpString::dump(ptr, depth)]
                }
            }
        )*
    };
}

impl_dump!(
    u8:"{:02X}", i8:"{:02X}",
    u16:"{:04X}", i16:"{:04X}",
    u32:"{:08X}", i32:"{:08X}",
    u64:"{:016X}", i64:"{:016X}",
    u128:"{:32X}", i128:"{:32X}"
);

impl DumpString for str {
    fn dump(&self, depth: usize) -> String {
        let mut padding = String::new();
        padding.reserve(depth);
        for _ in 0..depth {
            padding.push_str(__private::PAD_STR);
        }
        format!("{padding}{self}")
    }

    fn dump_as_lines(&self, depth: usize) -> Vec<String> {
        vec![self.dump(depth)]
    }
}

unsafe impl UnsafeDumpString for str {
    unsafe fn dump(ptr: *const Self, depth: usize) -> String {
        let str = if cfg!(debug_assertions) {
            ptr.as_ref().unwrap_unchecked()
        } else {
            ptr.as_ref().unwrap()
        };
        let mut padding = String::new();
        padding.reserve(depth);
        for _ in 0..depth {
            padding.push_str(__private::PAD_STR);
        }
        format!("{padding}{str}")
    }

    unsafe fn dump_as_lines(ptr: *const Self, depth: usize) -> Vec<String> {
        vec![UnsafeDumpString::dump(ptr, depth)]
    }
}
