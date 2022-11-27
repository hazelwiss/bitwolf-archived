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

    #[inline]
    pub fn set_lr(&mut self, val: u32) {
        self.set(14, val)
    }

    #[inline]
    pub fn get_lr(&mut self) -> u32 {
        self.get(14)
    }
}

/// Program status register.
pub struct Psr(u32);

macro_rules! get_bit {
    ($val:expr, $bit:literal) => {
        (($val >> $bit) & 0b1 != 0)
    };
}

macro_rules! set_bit {
    ($val:expr, $bit:literal, $bool:expr) => {{
        $val = ($val & !(1 << $bit)) | (($bool as u32) << $bit)
    }};
}

impl Psr {
    pub fn new() -> Self {
        Self(0)
    }

    /// Get carry.
    #[inline(always)]
    pub fn get_c(&self) -> bool {
        get_bit!(self.0, 29)
    }

    /// Set carry.
    #[inline(always)]
    pub fn set_c(&mut self, v: bool) {
        set_bit!(self.0, 29, v)
    }

    /// Get overflow.
    #[inline(always)]
    pub fn get_v(&mut self) -> bool {
        get_bit!(self.0, 28)
    }

    /// Set overflow.
    #[inline(always)]
    pub fn set_v(&mut self, v: bool) {
        set_bit!(self.0, 28, v)
    }

    /// Get zero.
    #[inline(always)]
    pub fn get_z(&mut self) -> bool {
        get_bit!(self.0, 30)
    }

    /// Set zero.
    #[inline(always)]
    pub fn set_z(&mut self, v: bool) {
        set_bit!(self.0, 30, v)
    }

    /// Get negative.
    #[inline(always)]
    pub fn get_n(&mut self) -> bool {
        get_bit!(self.0, 31)
    }

    /// Set negative.
    #[inline(always)]
    pub fn set_n(&mut self, v: bool) {
        set_bit!(self.0, 31, v)
    }

    #[inline(always)]
    pub fn raw(&self) -> u32 {
        self.0
    }
}
