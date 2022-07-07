pub mod instructions;
pub mod interrupt;
pub mod registers;

pub struct CPU {
    reg_file: registers::RegisterFile,
    ime: bool,
    halted: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            reg_file: registers::RegisterFile::new(),
            ime: false,
            halted: false,
        }
    }

    #[inline]
    pub fn ime_set(&mut self, val: bool) {
        self.ime = val;
    }

    #[inline]
    pub fn ime_get(&self) -> bool {
        self.ime
    }

    #[inline]
    pub fn halted_get(&self) -> bool {
        self.halted
    }

    #[inline]
    pub fn halted_set(&mut self, val: bool) {
        self.halted = val;
    }
}
