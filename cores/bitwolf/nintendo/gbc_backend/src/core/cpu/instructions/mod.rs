pub mod decode;

use crate::core::cpu::registers::{R16, R8};
use decode::{RSTVec, CC};

use self::decode::Bit;

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
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
    ADD_SP_I,
    CB,

    RST(RSTVec),

    PUSH(R16),
    POP(R16),

    CALLCC(CC),
    JPCC(CC),
    RETCC(CC),
    JRCC(CC),

    ADD_HL_R16(R16),
    ADD_HL_SP,

    LD_PNN_SP,
    LD_PHLI_A,
    LD_PHLD_A,
    LDH_A_PN,
    LDH_PN_A,
    LDH_A_PC,
    LDH_PC_A,

    LD_A_PHLI,
    LD_A_PHLD,

    LD_R8_R8(R8, R8),
    LD_R8_PHL(R8),
    LD_PHL_R8(R8),

    LD_R8_N(R8),
    LD_PHL_N,

    LD_R16_NN(R16),
    LD_SP_NN,

    LD_PR16_A(R16),
    LD_A_PR16(R16),

    LD_PNN_A,
    LD_A_PNN,

    LD_HL_SP_I,
    LD_SP_HL,

    INC_R8(R8),
    INC_PHL,
    INC_R16(R16),
    INC_SP,
    DEC_R8(R8),
    DEC_PHL,
    DEC_R16(R16),
    DEC_SP,

    ADD_N,
    ADD_R8(R8),
    ADD_PHL,

    ADC_N,
    ADC_R8(R8),
    ADC_PHL,

    SUB_N,
    SUB_R8(R8),
    SUB_PHL,

    SBC_N,
    SBC_R8(R8),
    SBC_PHL,

    AND_N,
    AND_R8(R8),
    AND_PHL,

    XOR_N,
    XOR_R8(R8),
    XOR_PHL,

    OR_N,
    OR_R8(R8),
    OR_PHL,

    CP_N,
    CP_R8(R8),
    CP_PHL,

    INVALID,
}

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Prefixed {
    RLC(R8),
    RLC_PHL,
    RRC(R8),
    RRC_PHL,
    RL(R8),
    RL_PHL,
    RR(R8),
    RR_PHL,
    SLA(R8),
    SLA_PHL,
    SRA(R8),
    SRA_PHL,
    SWAP(R8),
    SWAP_PHL,
    SRL(R8),
    SRL_PHL,
    BIT(Bit, R8),
    BIT_PHL(Bit),
    RES(Bit, R8),
    RES_PHL(Bit),
    SET(Bit, R8),
    SET_PHL(Bit),
}
