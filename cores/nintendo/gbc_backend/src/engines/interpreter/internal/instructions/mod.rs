mod defs;
mod lut;

use super::Interpreter;

impl Interpreter {
    pub fn fetch_decode_execute(&mut self) {
        let val = 0 as u8;
        lut::UNPREFIXED_INSTR_LUT[val as usize](self);
    }
}
