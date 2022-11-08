pub struct RegFile {
    pub gpr: [u32; 16],
}

impl RegFile {
    #[inline]
    pub fn get(&self, index: usize) -> u32 {
        debug_assert!(index < 0x10);
        unsafe { *self.gpr.get_unchecked(index & 0xF) }
    }

    #[inline]
    pub fn set(&mut self, index: usize, val: u32) {
        debug_assert!(index < 0x10);
        unsafe { *self.gpr.get_unchecked_mut(index & 0xF) = val };
    }

    #[inline]
    pub fn get_pc(&self) -> u32 {
        self.get(15)
    }

    #[inline]
    pub fn set_pc(&mut self, val: u32) {
        self.set(15, val);
    }
}
