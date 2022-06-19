use super::Unprefixed;
use crate::registers::{R16, R8};

#[derive(Debug, PartialEq, Eq)]
pub enum CC {
    NZ,
    Z,
    NC,
    C,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum LD {
    PNN_SP,
    PHLI_A,
    PHLD_A,
    H_A_PN,
    H_PN_A,
    H_A_PC,
    H_PC_A,

    A_PHLI,
    A_PHLD,
    E8_R8(E8, E8),
    E8_N(E8),
    R16_NN(RPTblEntry),
    PR16_R8(R16, R8),
    R8_PR16(R8, R16),
    PNN_A,
    A_PNN,
    HL_SP_D,
    SP_HL,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RSTVec {
    V00,
    V08,
    V10,
    V18,
    V20,
    V28,
    V30,
    V38,
}

#[derive(Debug, PartialEq, Eq)]
pub enum INC {
    E8(E8),
    RPTblRet(RPTblEntry),
}

#[derive(Debug, PartialEq, Eq)]
pub enum DEC {
    E8(E8),
    RPTblRet(RPTblEntry),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ROT {
    RLC(ALUArg),
    RRC(ALUArg),
    RL(ALUArg),
    RR(ALUArg),
    SLA(ALUArg),
    SRA(ALUArg),
    SWAP(ALUArg),
    SRL(ALUArg),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ALU {
    ADD(ALUArg),
    ADC(ALUArg),
    SUB(ALUArg),
    SBC(ALUArg),
    AND(ALUArg),
    XOR(ALUArg),
    OR(ALUArg),
    CP(ALUArg),
}

impl Unprefixed {
    pub const fn from_byte(byte: u8) -> Self {
        let x = byte >> 6;
        let y = (byte >> 3) & 0b111;
        let z = byte & 0b111;
        match x {
            0b00 => Self::x0(y, z),
            0b01 => Self::x1(y, z),
            0b10 => Self::x2(y, z),
            0b11 => Self::x3(y, z),
            _ => panic!("impossible decode x-value"),
        }
    }

    const fn x0(y: u8, z: u8) -> Self {
        const fn handle(y: u8, z: u8) -> Option<Unprefixed> {
            let q = y & 0b1 != 0;
            let p = y >> 1;
            Some(match z {
                0b000 => match y {
                    0 => Unprefixed::NOP,
                    1 => Unprefixed::LD(LD::PNN_SP),
                    2 => Unprefixed::STOP,
                    3 => Unprefixed::JR,
                    4..=7 => Unprefixed::JRCC(Unprefixed::tbl_cc(y & 0b11)),
                    _ => return None,
                },
                0b001 => match q {
                    false => Unprefixed::LD(LD::R16_NN(Unprefixed::tbl_rp(p))),
                    true => Unprefixed::ADDHL(Unprefixed::tbl_rp(p)),
                },
                0b010 => match q {
                    false => match p {
                        0 => Unprefixed::LD(LD::PR16_R8(R16::BC, R8::A)),
                        1 => Unprefixed::LD(LD::PR16_R8(R16::DE, R8::A)),
                        2 => Unprefixed::LD(LD::PHLI_A),
                        3 => Unprefixed::LD(LD::PHLD_A),
                        _ => return None,
                    },
                    true => match p {
                        0 => Unprefixed::LD(LD::R8_PR16(R8::A, R16::BC)),
                        1 => Unprefixed::LD(LD::R8_PR16(R8::A, R16::DE)),
                        2 => Unprefixed::LD(LD::A_PHLI),
                        3 => Unprefixed::LD(LD::A_PHLD),
                        _ => return None,
                    },
                },
                0b011 => match q {
                    false => Unprefixed::INC(INC::RPTblRet(Unprefixed::tbl_rp(p))),
                    true => Unprefixed::DEC(DEC::RPTblRet(Unprefixed::tbl_rp(p))),
                },
                0b100 => Unprefixed::INC(INC::E8(Unprefixed::tbl_r(y))),
                0b101 => Unprefixed::DEC(DEC::E8(Unprefixed::tbl_r(y))),
                0b110 => Unprefixed::LD(LD::E8_N(Unprefixed::tbl_r(y))),
                0b111 => match y {
                    0 => Unprefixed::RLCA,
                    1 => Unprefixed::RRCA,
                    2 => Unprefixed::RLA,
                    3 => Unprefixed::RRA,
                    4 => Unprefixed::DAA,
                    5 => Unprefixed::CPL,
                    6 => Unprefixed::SCF,
                    7 => Unprefixed::CCF,
                    _ => return None,
                },
                _ => return None,
            })
        }
        if let Some(val) = handle(y, z) {
            val
        } else {
            panic!("Unable to decode x=0")
        }
    }

    const fn x1(y: u8, z: u8) -> Self {
        match z {
            0b110 if y == 6 => Self::HALT,
            0b000..=0b111 => Self::LD(LD::E8_R8(Self::tbl_r(y), Self::tbl_r(z))),
            _ => panic!("impossible decode z-value"),
        }
    }

    const fn x2(y: u8, z: u8) -> Self {
        Self::ALU(Self::tbl_alu(y, ALUArg::E8(Self::tbl_r(z))))
    }

    const fn x3(y: u8, z: u8) -> Self {
        const fn handle(y: u8, z: u8) -> Option<Unprefixed> {
            let q = y & 0b1 != 0;
            let p = y >> 1;
            Some(match z {
                0b000 => match y {
                    0..=3 => Unprefixed::RETCC(Unprefixed::tbl_cc(y)),
                    4 => Unprefixed::LD(LD::H_PN_A),
                    5 => Unprefixed::ADDSP,
                    6 => Unprefixed::LD(LD::H_A_PN),
                    7 => Unprefixed::LD(LD::HL_SP_D),
                    _ => return None,
                },
                0b001 => match q {
                    false => Unprefixed::POP(Unprefixed::tbl_rp2(p)),
                    true => match p {
                        0 => Unprefixed::RET,
                        1 => Unprefixed::RETI,
                        2 => Unprefixed::JPHL,
                        3 => Unprefixed::LD(LD::SP_HL),
                        _ => return None,
                    },
                },
                0b010 => match y {
                    0..=3 => Unprefixed::JPCC(Unprefixed::tbl_cc(y)),
                    4 => Unprefixed::LD(LD::H_PC_A),
                    5 => Unprefixed::LD(LD::PNN_A),
                    6 => Unprefixed::LD(LD::H_A_PC),
                    7 => Unprefixed::LD(LD::A_PNN),
                    _ => return None,
                },
                0b011 => match y {
                    0 => Unprefixed::JP,
                    1 => Unprefixed::CB,
                    2..=5 => Unprefixed::INVALID,
                    6 => Unprefixed::DI,
                    7 => Unprefixed::EI,
                    _ => return None,
                },
                0b100 => match y {
                    0..=3 => Unprefixed::CALLCC(Unprefixed::tbl_cc(y)),
                    4..=7 => Unprefixed::INVALID,
                    _ => return None,
                },
                0b101 => match q {
                    false => Unprefixed::PUSH(Unprefixed::tbl_rp2(p)),
                    true => match p {
                        0 => Unprefixed::CALL,
                        1..=3 => Unprefixed::INVALID,
                        _ => return None,
                    },
                },
                0b110 => Unprefixed::ALU(Unprefixed::tbl_alu(y, ALUArg::N)),
                0b111 => Unprefixed::RST(match y {
                    0 => RSTVec::V00,
                    1 => RSTVec::V08,
                    2 => RSTVec::V10,
                    3 => RSTVec::V18,
                    4 => RSTVec::V20,
                    5 => RSTVec::V28,
                    6 => RSTVec::V30,
                    7 => RSTVec::V38,
                    _ => return None,
                }),
                _ => panic!("impossible decode z-value"),
            })
        }
        if let Some(val) = handle(y, z) {
            val
        } else {
            panic!("Unable to decode for x=3")
        }
    }

    const fn tbl_r(idx: u8) -> E8 {
        E8::R8(match idx {
            0 => R8::B,
            1 => R8::C,
            2 => R8::D,
            3 => R8::E,
            4 => R8::H,
            5 => R8::L,
            6 => return E8::PHL,
            7 => R8::A,
            _ => panic!("Invalid index for r LUT lookup"),
        })
    }

    const fn tbl_rp(idx: u8) -> RPTblEntry {
        RPTblEntry::R16(match idx {
            0 => R16::BC,
            1 => R16::DE,
            2 => R16::HL,
            3 => return RPTblEntry::SP,
            _ => panic!("Invalid index for rp LUT lookup"),
        })
    }

    const fn tbl_rp2(idx: u8) -> R16 {
        match idx {
            0 => R16::BC,
            1 => R16::DE,
            2 => R16::HL,
            3 => R16::AF,
            _ => panic!("Invalid index for rp2 LUT lookup"),
        }
    }

    const fn tbl_cc(idx: u8) -> CC {
        match idx {
            0 => CC::NZ,
            1 => CC::Z,
            2 => CC::NC,
            3 => CC::C,
            _ => panic!("Invalid index for cc LUT lookup"),
        }
    }

    const fn tbl_alu(idx: u8, src: ALUArg) -> ALU {
        match idx {
            0 => ALU::ADD(src),
            1 => ALU::ADC(src),
            2 => ALU::SUB(src),
            3 => ALU::SBC(src),
            4 => ALU::AND(src),
            5 => ALU::XOR(src),
            6 => ALU::OR(src),
            7 => ALU::CP(src),
            _ => panic!(""),
        }
    }

    //const fn tbl_rot(idx: u8) -> ROT {}
}

#[derive(Debug, PartialEq, Eq)]
pub enum ALUArg {
    E8(E8),
    N,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RPTblEntry {
    R16(R16),
    SP,
}

#[derive(Debug, PartialEq, Eq)]
pub enum E8 {
    R8(R8),
    PHL,
}
