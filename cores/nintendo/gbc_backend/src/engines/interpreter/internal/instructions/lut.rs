use super::Interpreter;
use cpu::instrutions::Unprefixed;

pub type InstrFunc = fn(&mut Interpreter);
pub static PREFIXED_INSTR_LUT: [InstrFunc; 256] = {
    const fn fill_instr_table(lut: &mut [InstrFunc; 256], idx: usize) {
        let instr = match true {
            _ => filler,
        };
        if idx < u8::MAX as usize {
            fill_instr_table(lut, idx + 1)
        }
    }
    let mut lut = [filler as InstrFunc; 256];
    fill_instr_table(&mut lut, 0);
    lut
};
pub static UNPREFIXED_INSTR_LUT: [InstrFunc; 256] = {
    const fn fill_instr_table(lut: &mut [InstrFunc; 256], idx: usize) {
        use cpu::instrutions::decode::RSTVec;
        let val = Unprefixed::from_byte(idx as u8);
        let instr = match val {
            Unprefixed::NOP => Interpreter::nop,
            Unprefixed::STOP => Interpreter::stop,
            Unprefixed::RLCA => Interpreter::rlca,
            Unprefixed::RRCA => Interpreter::rrca,
            Unprefixed::RLA => Interpreter::rla,
            Unprefixed::RRA => Interpreter::rra,
            Unprefixed::DAA => Interpreter::daa,
            Unprefixed::CPL => Interpreter::cpl,
            Unprefixed::SCF => Interpreter::scf,
            Unprefixed::CCF => Interpreter::ccf,
            Unprefixed::JR => Interpreter::jr,
            Unprefixed::HALT => Interpreter::halt,
            Unprefixed::RET => Interpreter::ret,
            Unprefixed::RETI => Interpreter::reti,
            Unprefixed::JPHL => Interpreter::jp_hl,
            Unprefixed::JP => Interpreter::jp,
            Unprefixed::DI => Interpreter::di,
            Unprefixed::EI => Interpreter::ei,
            Unprefixed::CALL => Interpreter::call,
            Unprefixed::ADDSP => Interpreter::add_sp_e8,
            Unprefixed::CB => Interpreter::nop, // temporar
            Unprefixed::RST(vec) => {
                todo!()
                //macros::functions::match_variations!(Interpreter::rst::<vec>)
            }
            Unprefixed::PUSH(reg) => todo!(),
            Unprefixed::POP(reg) => todo!(),
            Unprefixed::CALLCC(reg) => todo!(),
            Unprefixed::JPCC(cc) => todo!(),
            Unprefixed::ADDHL(cc) => todo!(),
            Unprefixed::RETCC(cc) => todo!(),
            Unprefixed::JRCC(cc) => todo!(),
            Unprefixed::LD(_) => todo!(),
            Unprefixed::INC(_) => todo!(),
            Unprefixed::DEC(_) => todo!(),
            Unprefixed::ALU(_) => todo!(),
            Unprefixed::ROT(_) => todo!(),
            Unprefixed::INVALID => todo!(),
        };
        if idx < u8::MAX as usize {
            fill_instr_table(lut, idx + 1)
        }
    }
    let mut lut = [filler as InstrFunc; 256];
    fill_instr_table(&mut lut, 0);
    lut
};

// Initial value in the LUT to be filled.
fn filler(_: &mut Interpreter) {
    panic!("Filler function executed.")
}
