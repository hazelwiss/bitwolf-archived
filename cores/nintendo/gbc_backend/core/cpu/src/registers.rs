use super::CPU;

#[derive(Debug)]
pub enum R8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
pub enum R16 {
    AF,
    BC,
    DE,
    HL,
}

#[derive(Debug)]
pub enum Flag {
    Z,
    N,
    H,
    C,
}

#[derive(Debug)]
pub struct FlagRegister {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

impl FlagRegister {
    pub fn get(&self, f: Flag) -> bool {
        match f {
            Flag::Z => self.z,
            Flag::N => self.n,
            Flag::H => self.h,
            Flag::C => self.c,
        }
    }

    pub fn set(&mut self, f: Flag, v: bool) {
        match f {
            Flag::Z => self.z = v,
            Flag::N => self.n = v,
            Flag::H => self.h = v,
            Flag::C => self.c = v,
        }
    }

    fn new(val: u8) -> Self {
        Self {
            z: val & 0b1000_0000 != 0,
            n: val & 0b0100_0000 != 0,
            h: val & 0b0010_0000 != 0,
            c: val & 0b0001_0000 != 0,
        }
    }

    fn as_u8(&self) -> u8 {
        (self.z as u8) << 7 | (self.n as u8) << 6 | (self.h as u8) << 5 | (self.c as u8) << 4
    }
}

#[derive(Debug)]
pub struct RegisterFile {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: FlagRegister,
    pc: u16,
    sp: u16,
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: FlagRegister::new(0),
            sp: 0,
            pc: 0,
        }
    }

    #[inline]
    pub fn read_sp(&self) -> u16 {
        self.sp
    }

    #[inline]
    pub fn write_sp(&mut self, v: u16) {
        self.sp = v
    }

    #[inline]
    pub fn read_pc(&self) -> u16 {
        self.pc
    }

    #[inline]
    pub fn write_pc(&mut self, v: u16) {
        self.pc = v
    }

    #[inline]
    pub fn set_flag(&mut self, f: Flag, v: bool) {
        self.f.set(f, v)
    }

    #[inline]
    pub fn get_flag(&self, f: Flag) -> bool {
        self.f.get(f)
    }

    #[inline]
    pub fn read_r8(&self, src: R8) -> u8 {
        match src {
            R8::A => self.a,
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::H => self.h,
            R8::L => self.l,
        }
    }

    #[inline]
    pub fn write_r8(&mut self, dst: R8, v: u8) {
        match dst {
            R8::A => self.a = v,
            R8::B => self.b = v,
            R8::C => self.c = v,
            R8::D => self.d = v,
            R8::E => self.e = v,
            R8::H => self.h = v,
            R8::L => self.l = v,
        }
    }

    #[inline]
    pub fn read_r16(&self, src: R16) -> u16 {
        match src {
            R16::AF => (self.a as u16) << 8 | self.f.as_u8() as u16,
            R16::BC => (self.b as u16) << 8 | self.c as u16,
            R16::DE => (self.d as u16) << 8 | self.d as u16,
            R16::HL => (self.h as u16) << 8 | (self.l as u16),
        }
    }

    #[inline]
    pub fn write_r16(&mut self, dst: R16, v: u16) {
        let hi = (v >> 8) as u8;
        let lo = v as u8;
        match dst {
            R16::AF => {
                self.a = hi;
                self.f = FlagRegister::new(lo);
            }
            R16::BC => {
                self.b = hi;
                self.c = lo;
            }
            R16::DE => {
                self.d = hi;
                self.e = lo;
            }
            R16::HL => {
                self.h = hi;
                self.l = lo;
            }
        }
    }
}

impl CPU {
    pub fn regs(&self) -> &RegisterFile {
        &self.reg_file
    }
    pub fn regs_mut(&mut self) -> &mut RegisterFile {
        &mut self.reg_file
    }
}
