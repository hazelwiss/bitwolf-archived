/// A byte array contaniner for operating on byte values.
pub struct Bytes<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> Bytes<SIZE> {}

/// A safe container for operating on the `Bytes` type.
/// Implements pointer logic in a safe manner which
/// respects the stacked borrows model.
pub struct BytesCell<const SIZE: usize>(*mut Bytes<SIZE>);

impl<const SIZE: usize> BytesCell<SIZE> {
    pub fn new_zeroed() -> Self {
        unsafe { Self(alloc::alloc::alloc_zeroed(alloc::alloc::Layout::new::<Bytes<SIZE>>()) as _) }
    }

    /// # Safety
    /// bytes has to be a valid pointer of
    /// the `Bytes` type and non-null.
    pub unsafe fn from_raw(raw: *mut Bytes<SIZE>) -> Self {
        Self(raw)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
