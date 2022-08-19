use std::{
    alloc::Layout,
    ops::{Deref, DerefMut},
};

///
pub struct Bytes<const LEN: usize>([u8; LEN]);

impl<const LEN: usize> Bytes<LEN> {
    #[inline]
    pub const fn new(bytes: [u8; LEN]) -> Self {
        Self(bytes)
    }

    #[inline]
    pub fn new_zeroed() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        LEN
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_array_ptr(&self) -> *mut [u8; LEN] {
        &self as *const _ as *mut _
    }
}

impl<const LEN: usize> Deref for Bytes<LEN> {
    type Target = [u8; LEN];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const LEN: usize> DerefMut for Bytes<LEN> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

///
pub struct OwnedBytesCell<const LEN: usize>(*mut Bytes<LEN>);

impl<const LEN: usize> OwnedBytesCell<LEN> {
    #[inline]
    pub fn new(bytes: [u8; LEN]) -> Self {
        unsafe {
            let ptr = std::alloc::alloc(Layout::new::<Bytes<LEN>>());
            ptr.copy_from(bytes.as_ptr(), LEN);
            Self(ptr as *mut Bytes<LEN>)
        }
    }

    #[inline]
    pub fn new_zeroed() -> Self {
        unsafe { Self(std::alloc::alloc_zeroed(Layout::new::<Bytes<LEN>>()) as *mut Bytes<LEN>) }
    }

    #[inline]
    pub fn into_inner(self) -> *mut Bytes<LEN> {
        self.0
    }

    #[inline]
    pub const fn len(&self) -> usize {
        LEN
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
