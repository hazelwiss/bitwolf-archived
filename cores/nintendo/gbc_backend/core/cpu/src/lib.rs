pub mod instrutions;
pub mod registers;

pub struct CPU {
    reg_file: registers::RegisterFile,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            reg_file: registers::RegisterFile::new(),
        }
    }
}
