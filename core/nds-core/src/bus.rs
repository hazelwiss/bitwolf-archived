pub mod access;

mod read;
mod write;

pub use read::{read16, read32, read8};
pub use write::{write16, write32, write8};

pub trait Pagable {
    fn page_from_index(index: u32) -> *mut u8;
}
