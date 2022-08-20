pub use macros::{DumpString, UnsafeDumpString};

pub trait DumpString {
    fn dump(&self) -> String;

    fn dump_as_lines(&self) -> Vec<String>;
}

pub unsafe trait UnsafeDumpString {
    unsafe fn dump(&self) -> String;

    unsafe fn dump_as_lines(&self) -> Vec<String>;
}
