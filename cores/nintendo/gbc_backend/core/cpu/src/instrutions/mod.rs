pub mod decode;
//#[cfg(feature = "test")]
pub mod test;

use crate::registers::R16;

#[derive(Debug)]
pub enum Unprefixed {
    NOP,
    STOP,
    RLCA,
    RRCA,
    RLA,
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
    JR,
    HALT,
    RET,
    RETI,
    JPHL,
    JP,
    DI,
    EI,
    CALL,
    ADDSP,
    CB,
    RST(decode::RSTVec),
    PUSH(R16),
    POP(R16),
    CALLCC(decode::CC),
    JPCC(decode::CC),
    ADDHL(decode::RPTblEntry),
    RETCC(decode::CC),
    JRCC(decode::CC),
    LD(decode::LD),
    INC(decode::INC),
    DEC(decode::DEC),
    ALU(decode::ALU),
    ROT(decode::ROT),
    INVALID,
}
