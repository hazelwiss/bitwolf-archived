use std::{
    alloc::Layout,
    ops::{Deref, DerefMut, Index, IndexMut, Range},
};

macro_rules! impl_reads {
    () => {
        #[inline]
        pub fn read(&self, offset: usize) -> u8 {
            debug_assert!(offset < self.len());
            unsafe { *self.as_ptr().add(offset) }
        }
    };
}

macro_rules! impl_writes {
    () => {
        #[inline]
        pub fn write(&mut self, offset: usize, val: u8) {
            debug_assert!(offset < self.len());
            unsafe { *self.as_mut_ptr().add(offset) = val };
        }
    };
}

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

    impl_reads!();
    impl_writes!();
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

impl<const LEN: usize> Index<usize> for Bytes<LEN> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const LEN: usize> IndexMut<usize> for Bytes<LEN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
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

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        unsafe { self.0.as_ref().unwrap_unchecked().as_ptr() }
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        unsafe { (self.0).as_mut().unwrap_unchecked().as_mut_ptr() }
    }

    #[inline]
    pub fn as_slice(&self, range: Range<usize>) -> OwnedByteSlice {
        let ptr = unsafe { (*self.0).as_mut_ptr().add(range.start) };
        OwnedByteSlice::from_raw(ptr, range.len())
    }

    impl_reads!();
    impl_writes!();
}

impl<const LEN: usize> Index<usize> for OwnedBytesCell<LEN> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        let val = unsafe { self.0.as_ref().unwrap_unchecked() };
        &val[index]
    }
}

impl<const LEN: usize> IndexMut<usize> for OwnedBytesCell<LEN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = unsafe { self.0.as_mut().unwrap_unchecked() };
        &mut val[index]
    }
}

///
pub struct OwnedByteSlice(*mut u8, usize);

impl OwnedByteSlice {
    #[inline]
    pub fn from_raw(ptr: *mut u8, len: usize) -> Self {
        Self(ptr, len)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.1
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.0
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0
    }

    impl_reads!();
    impl_writes!();
}
