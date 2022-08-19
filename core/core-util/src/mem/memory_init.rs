//use std::alloc::{alloc_zeroed, Layout};
//
///// # Safety
///// Self has to be valid when filled
///// with zeroes.
//pub unsafe trait MemZero {
//    #[inline]
//    unsafe fn fill_zeroed(ptr: *mut Self)
//    where
//        Self: Sized,
//    {
//        *ptr = std::mem::MaybeUninit::zeroed().assume_init()
//    }
//}
//
///// # Safety
///// Self has to be valid when filled
///// with arbitrary values.
//pub unsafe trait MemFill8 {
//    #[inline]
//    unsafe fn fill_val(ptr: *mut Self, v: u8)
//    where
//        Self: Sized,
//    {
//        ptr.write_bytes(v, std::mem::size_of::<Self>());
//    }
//}
//
//unsafe impl<const LEN: usize> MemZero for [u8; LEN] {}
//unsafe impl MemZero for [u8] {}
//unsafe impl<T> MemZero for *mut T {}
//unsafe impl<T> MemZero for *const T {}
//
//unsafe impl<const LEN: usize> MemFill8 for [u8; LEN] {}
//unsafe impl MemFill8 for [u8] {}
//
///// # Safety
///// T must be valid if filled with zeroes.
//#[inline]
//pub unsafe fn allocate_zeroed<T: MemZero>() -> *mut T {
//    alloc_zeroed(Layout::new::<T>()) as *mut T
//}
//
///// # Safety
///// T must be valid if filled with any arbirary value.
//#[inline]
//pub unsafe fn allocate_fill<T: MemFill8>(b: u8) -> *mut T {
//    let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
//    MemFill8::fill_val(ptr, b);
//    ptr
//}
//
///// # Safety
///// T must be valid if filled with zeroes.
//#[inline]
//pub unsafe fn zeroed<T: MemZero>() -> T {
//    std::mem::MaybeUninit::zeroed().assume_init()
//}
//
///// # Safety
///// T must be valid if filled with any arbirary value.
//#[inline]
//pub unsafe fn fill<T: MemFill8>(b: u8) -> T {
//    let mut uninit: T = std::mem::MaybeUninit::uninit().assume_init();
//    T::fill_val(&mut uninit as *mut _, b);
//    uninit
//}
//
