#[inline(always)]
pub unsafe fn to_byte_slice<T>(data: &[T]) -> &[u8] {
    std::slice::from_raw_parts(
        data.as_ptr() as *const u8,
        std::mem::size_of::<T>() * data.len(),
    )
}
