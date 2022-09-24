pub struct Processor {}

#[allow(clippy::enum_variant_names)]
pub enum DSPMulTy {
    SMUL { x: bool },
    SMLA { x: bool },
    SMULW,
    SMLAW,
    SMLAL { x: bool },
}

#[allow(clippy::enum_variant_names)]
pub enum DPOpcTy {
    AND,
    EOR,
    SUB,
    RSB,
    ADD,
    ADC,
    SBC,
    RSC,
    TST,
    TEQ,
    CMP,
    CMN,
    ORR,
    MOV,
    BIC,
    MVN,
}

#[allow(clippy::enum_variant_names)]
pub enum DpShftTy {
    LSL,
    LSR,
    ASR,
    ROR,
}

pub enum DpOperTy {
    Imm,
    Shft { reg: bool, ty: DpShftTy },
}

#[allow(clippy::enum_variant_names)]
pub enum Instr {
    MSR {
        r: bool,
        imm: bool,
    },
    MRS {
        r: bool,
    },
    BX,
    BLX,
    CLZ,
    SatAddSub {
        sub: bool,
        doubles: bool,
    },
    DSPMul {
        ty: DSPMulTy,
        y: bool,
    },
    BKPT,
    DP {
        set_flags: bool,
        opcode: DPOpcTy,
        operand: DpOperTy,
    },
    Undef,
}

macro_rules! b {
    ($b:literal) => {
        (1 << $b)
    };
}

impl Processor {
    pub fn decode_cond(instr: u32) -> Instr {
        let instr = instr & 0x0FFFFFFF;
        // Miscellaneous instructions (3-3)
        if instr & 0x0F90_0010 == 0x0100_0000 || instr & 0x0F90_0090 == 0x0100_0010 {
            // Some miscelanous instructions must be handled before DP instructions due
            // to a 'hole' within the encoding table caused by the opcode field being equal
            // to 0b10xx while S is zero.
            let bit_7 = instr & b!(7) != 0;
            let bits = (instr >> 4) & 0b111;
            let upper = (instr >> 21) & 0b11;
            if bit_7 {
                match bits {
                    0b000 => {
                        let r = upper & b!(1) != 0;
                        // Move status register to register.
                        if bits & 0b01 == 0 {
                            Instr::MRS { r }
                        }
                        // Move register to status register.
                        else {
                            Instr::MSR { r, imm: false }
                        }
                    }
                    0b001 => {
                        // Branch/exchange instruction set.
                        if upper == 0b01 {
                            Instr::BX
                        }
                        // Count leading zeros.
                        else if upper == 0b11 {
                            Instr::CLZ
                        } else {
                            Instr::Undef
                        }
                    }
                    0b011 => {
                        // Branch and link/exchange instruction set.
                        if upper == 0b01 {
                            Instr::BLX
                        } else {
                            Instr::Undef
                        }
                    }
                    0b101 => {
                        // Enhanced DSP add/subtracts.
                        let sub = upper & b!(1) != 0;
                        let doubles = upper & b!(0) != 0;
                        Instr::SatAddSub { sub, doubles }
                    }
                    0b111 => {
                        // Software Breakpoint.
                        if upper == 0b01 {
                            Instr::BKPT
                        } else {
                            Instr::Undef
                        }
                    }
                    _ => Instr::Undef,
                }
            } else {
                // Enhanced DSP multiples.
                if bits & 0b001 == 0 {
                    let x = bits & 0b010 != 0;
                    let y = bits & 0b100 != 0;
                    Instr::DSPMul {
                        ty: match upper {
                            0b00 => DSPMulTy::SMLA { x },
                            0b01 => {
                                if x {
                                    DSPMulTy::SMULW
                                } else {
                                    DSPMulTy::SMLAW
                                }
                            }
                            0b10 => DSPMulTy::SMLAL { x },
                            0b11 => DSPMulTy::SMUL { x },
                            _ => panic!("unreachable"),
                        },
                        y,
                    }
                } else {
                    Instr::Undef
                }
            }
        }
        // Undefined instruction.
        else if instr & 0x0FB0_0000 == 0x0300_0000 {
            Instr::Undef
        }
        // Move immediate to status register
        else if instr & 0x0FB0_0000 == 0x0320_0000 {
            Instr::MSR {
                r: instr & b!(22) != 0,
                imm: true,
            }
        }
        // Data processing shift or immediate
        else if instr & 0x0E00_0010 == 0x0000_0000
            || instr & 0x0E00_0090 == 0x0000_0010
            || instr & 0x0E00_0000 == 0x0200_0000
        {
            let opcode = {
                use DPOpcTy::*;
                match (instr >> 21) & 0xF {
                    0b0000 => AND,
                    0b0001 => EOR,
                    0b0010 => SUB,
                    0b0011 => RSB,
                    0b0100 => ADD,
                    0b0101 => ADC,
                    0b0110 => SBC,
                    0b0111 => RSC,
                    0b1000 => TST,
                    0b1001 => TEQ,
                    0b1010 => CMP,
                    0b1011 => CMN,
                    0b1100 => ORR,
                    0b1101 => MOV,
                    0b1110 => BIC,
                    0b1111 => MVN,
                    _ => panic!("Unreachable"),
                }
            };
            let set_flags = instr & b!(20) != 0;
            let operand = if instr & b!(25) == 0 {
                let reg = instr & b!(4) != 0;
                let operand = DpOperTy::Shft {
                    reg,
                    ty: {
                        use DpShftTy::*;
                        match (instr >> 5) & 0b11 {
                            0b00 => LSL,
                            0b01 => LSR,
                            0b10 => ASR,
                            0b11 => ROR,
                            _ => panic!("unreachable!"),
                        }
                    },
                };
                operand
            } else {
                DpOperTy::Imm
            };
            Instr::DP {
                set_flags,
                opcode,
                operand,
            }
        }
        // Multiples, extra load/stores
        else if instr & 0x0E00_0090 == 0x0000_0090 {
            // p == 0
            //  indicates the use of post-indexed addressing. The base register value is used for
            //  the memory address, and the offset is then applied to the base register value and
            //  written back to the base register.
            // p == 1
            //  Indicates the use of offset addressing or pre-indexed addressing (the W bit
            //  determines which). The memory address is generated by applying the offset to
            //  the base register value.
            // U
            //  Indicates whether the offset is added to the base (U == 1) or subtracted from the base
            //  (U == 0).
            // W if p == 0
            //  The W bit must be 0 or the instruction is UNPREDICTABLE.
            // W if p == 1
            //  W == 1 indicates that the memory address is written back to the base register
            //  (pre-indexed addressing), and W == 0 that the base register is unchanged (offset
            //  addressing).
            todo!()
        }
        // Undefined Instruction
        else if instr & 0x0FB00_0000 == 0x0300_0000 {
            Instr::Undef
        }
        // Move immediate to status register
        else if instr & 0x0FB0_0000 == 0x0320_0000 {
            todo!()
        }
        // Load/store immediate offset
        else if instr & 0x0E00_0000 == 0x0400_0000 {
            todo!()
        }
        // Load/store register offset
        else if instr & 0x0600_0010 == 0x0600_0000 {
            todo!()
        }
        // Undefined instruction
        else if instr & 0x0600_0010 == 0x0600_0010 {
            Instr::Undef
        }
        // Load/store multiple
        else if instr & 0x0800_0000 == 0x0800_0000 {
            todo!()
        }
        // Branch and branch with link
        else if instr & 0x0E00_0000 == 0x0A00_0000 {
            todo!()
        }
        // Coprocessor load/store and double register transfers
        else if instr & 0x0E00_0000 == 0x0C00_0000 {
            todo!()
        }
        // Coprocessor data processing
        else if instr & 0x0F00_0010 == 0x0E00_0000 {
            todo!()
        }
        // Coprocessor register transfers
        else if instr & 0x0F00_0010 == 0x0E00_0010 {
            todo!()
        }
        // Software interrupt
        else if instr & 0x0F00_0000 == 0x0F00_0000 {
            todo!()
        }
        // Undefined
        else {
            Instr::Undef
        }
    }

    pub fn decode_uncond(_instr: u32) -> Instr {
        todo!()
    }
}
