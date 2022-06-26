#[derive(Clone, Copy, Default, Debug)]
#[repr(C, packed)]
pub struct BGRA(pub u8, pub u8, pub u8, pub u8);

impl BGRA {
    pub const WHITE: BGRA = BGRA::new(0xFF, 0xFF, 0xFF, 0xFF);
    pub const BLACK: BGRA = BGRA::new(0x00, 0x00, 0x00, 0x00);
    pub const RED: BGRA = BGRA::new(0x00, 0x00, 0xFF, 0xFF);
    pub const BLUE: BGRA = BGRA::new(0xFF, 0x00, 0x00, 0xFF);
    pub const GREEN: BGRA = BGRA::new(0x00, 0xFF, 0x00, 0xFF);

    #[inline(always)]
    pub const fn new(b: u8, g: u8, r: u8, a: u8) -> BGRA {
        BGRA(b, g, r, a)
    }

    #[inline(always)]
    pub fn full(&self) -> u32 {
        ((self.0 as u32) << 24) | ((self.1 as u32) << 16) | ((self.2 as u32) << 8) | (self.3 as u32)
    }

    #[inline(always)]
    pub fn r(&self) -> u8 {
        self.2
    }

    #[inline(always)]
    pub fn g(&self) -> u8 {
        self.1
    }

    #[inline(always)]
    pub fn b(&self) -> u8 {
        self.0
    }
}
