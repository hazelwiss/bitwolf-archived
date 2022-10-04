pub struct Processor {}

pub enum DSPMulTy {
    Smul { x: bool },
    Smla { x: bool },
    Smulw,
    Smlaw,
    Smlal { x: bool },
}

#[allow(clippy::enum_variant_names)]
pub enum DPOpcTy {
    And,
    Eor,
    Sub,
    Rsb,
    Add,
    Adc,
    Sbc,
    Rsc,
    Tst,
    Teq,
    Cmp,
    Cmn,
    Orr,
    Mov,
    Bic,
    Mvn,
}

#[allow(clippy::enum_variant_names)]
pub enum ShiftTy {
    LSL,
    LSR,
    ASR,
    ROR,
}

impl ShiftTy {
    fn from_bits(bits: u32) -> Self {
        assert!(bits > 0b100);
        use ShiftTy::*;
        match bits {
            0b00 => LSL,
            0b01 => LSR,
            0b10 => ASR,
            0b11 => ROR,
            _ => panic!("unrachable"),
        }
    }
}

pub enum TrasnfAdrTy {
    Post,
    Pre,
    Offset,
}

impl TrasnfAdrTy {
    fn from_w_p(w: bool, p: bool) -> TrasnfAdrTy {
        use TrasnfAdrTy::*;
        if p {
            if w {
                Pre
            } else {
                Offset
            }
        } else {
            Post
        }
    }
}

pub enum DpOperTy {
    Imm,
    Shft { reg: bool, ty: ShiftTy },
}

pub enum TransferOperTy {
    Imm,
    Reg { shift: ShiftTy },
}

pub enum MulTy {
    Mul,
    Mull { unsigned: bool },
}

pub enum Instr {
    Msr {
        r: bool,
        imm: bool,
    },
    Mrs {
        r: bool,
    },
    Bx,
    Blx {
        imm: bool,
    },
    Branch {
        link: bool,
    },
    Clz,
    SatAddSub {
        sub: bool,
        doubles: bool,
    },
    DspMul {
        ty: DSPMulTy,
        y: bool,
    },
    Bkpt,
    Dp {
        set_flags: bool,
        opcode: DPOpcTy,
        operand: DpOperTy,
    },
    Mul {
        acc: bool,
        set_flags: bool,
        ty: MulTy,
    },
    Swp {
        byte: bool,
    },
    Transfer {
        load: bool,
        byte: bool,
        offset_add: bool,
        operand: TransferOperTy,
        addressing: TrasnfAdrTy,
    },
    MiscTransfer {
        load: bool,
        signed: bool,
        halfword: bool,
        offset_add: bool,
        imm: bool,
        addressing: TrasnfAdrTy,
    },
    TransferDouble {
        store: bool,
        offset_add: bool,
        imm: bool,
        addressing: TrasnfAdrTy,
    },
    TransferMult {
        load: bool,
        update_base: bool,
        upwards: bool,
        privilige_mode: bool,
        exclude_first: bool,
    },
    CPTransfer {},
    CPDp {},
    CPRegTransfer {},
    Swi,
    Undef,
    Unpred,
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
                            Instr::Mrs { r }
                        }
                        // Move register to status register.
                        else {
                            Instr::Msr { r, imm: false }
                        }
                    }
                    0b001 => {
                        // Branch/exchange instruction set.
                        if upper == 0b01 {
                            Instr::Bx
                        }
                        // Count leading zeros.
                        else if upper == 0b11 {
                            Instr::Clz
                        } else {
                            Instr::Undef
                        }
                    }
                    0b011 => {
                        // Branch and link/exchange instruction set.
                        if upper == 0b01 {
                            Instr::Blx { imm: false }
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
                            Instr::Bkpt
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
                    Instr::DspMul {
                        ty: match upper {
                            0b00 => DSPMulTy::Smla { x },
                            0b01 => {
                                if x {
                                    DSPMulTy::Smulw
                                } else {
                                    DSPMulTy::Smlaw
                                }
                            }
                            0b10 => DSPMulTy::Smlal { x },
                            0b11 => DSPMulTy::Smul { x },
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
            Instr::Msr {
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
                    0b0000 => And,
                    0b0001 => Eor,
                    0b0010 => Sub,
                    0b0011 => Rsb,
                    0b0100 => Add,
                    0b0101 => Adc,
                    0b0110 => Sbc,
                    0b0111 => Rsc,
                    0b1000 => Tst,
                    0b1001 => Teq,
                    0b1010 => Cmp,
                    0b1011 => Cmn,
                    0b1100 => Orr,
                    0b1101 => Mov,
                    0b1110 => Bic,
                    0b1111 => Mvn,
                    _ => panic!("Unreachable"),
                }
            };
            let set_flags = instr & b!(20) != 0;
            let operand = if instr & b!(25) == 0 {
                let reg = instr & b!(4) != 0;
                let operand = DpOperTy::Shft {
                    reg,
                    ty: ShiftTy::from_bits((instr >> 5) & 0b11),
                };
                operand
            } else {
                DpOperTy::Imm
            };
            Instr::Dp {
                set_flags,
                opcode,
                operand,
            }
        }
        // Multiples, extra load/stores
        else if instr & 0x0E00_0090 == 0x0000_0090 {
            let bits = (instr >> 4) & 0b1111;
            match bits {
                0b1001 => {
                    let bits = (instr >> 20) & 0b11111;
                    if instr & b!(24) != 0 {
                        let bits = (instr >> 22) & 0b11;
                        let acc = instr & b!(21) != 0;
                        let set_flags = instr & b!(20) != 0;
                        let ty = if bits & 0b11 == 0b00 {
                            MulTy::Mul
                        } else if bits & 0b10 == 0b10 {
                            MulTy::Mull {
                                unsigned: instr & b!(22) != 0,
                            }
                        } else {
                            return Instr::Undef;
                        };
                        Instr::Mul { acc, set_flags, ty }
                    }
                    // Swap/swap byte.
                    else if bits & 0b11011 == 0b10000 {
                        Instr::Swp {
                            byte: instr & b!(22) != 0,
                        }
                    } else {
                        Instr::Undef
                    }
                }
                _ if bits & 0b1001 == 0b1001 => {
                    let b20 = instr & b!(20) != 0;
                    let offset_add = instr & b!(23) != 0;
                    let imm = instr & b!(22) != 0;
                    let w = instr & b!(21) != 0;
                    let p = instr & b!(24) != 0;
                    if p && w {
                        return Instr::Unpred;
                    }
                    let addressing = TrasnfAdrTy::from_w_p(w, p);
                    let bits = (instr >> 5) & 0b11;
                    if bits == 0b01 {
                        Instr::MiscTransfer {
                            load: b20,
                            signed: false,
                            halfword: true,
                            offset_add,
                            imm,
                            addressing,
                        }
                    } else if bits & 0b10 == 0b10 {
                        let imm = instr & b!(22) != 0;
                        // Load signed halfword/byte.
                        if b20 {
                            let halfword = instr & b!(5) != 0;
                            Instr::MiscTransfer {
                                load: true,
                                signed: true,
                                halfword,
                                offset_add,
                                imm,
                                addressing,
                            }
                        }
                        // Load/store two words.
                        else {
                            let store = instr & b!(5) != 0;
                            Instr::TransferDouble {
                                store,
                                imm,
                                offset_add,
                                addressing,
                            }
                        }
                    } else {
                        Instr::Undef
                    }
                }
                _ => Instr::Undef,
            }
        }
        // Load/store immediate/ offset
        else if instr & 0x0E00_0000 == 0x0400_0000 || instr & 0x0600_0010 == 0x0600_0000 {
            let w = instr & b!(21) != 0;
            let p = instr & b!(24) != 0;
            let addressing = TrasnfAdrTy::from_w_p(w, p);
            Instr::Transfer {
                load: instr & b!(20) != 0,
                byte: instr & b!(22) != 0,
                offset_add: instr & b!(23) != 0,
                operand: if instr & b!(25) != 0 {
                    // register
                    TransferOperTy::Reg {
                        shift: ShiftTy::from_bits((instr >> 5) & 0b11),
                    }
                } else {
                    // immediate
                    TransferOperTy::Imm
                },
                addressing,
            }
        }
        // Undefined instruction
        else if instr & 0x0600_0010 == 0x0600_0010 {
            Instr::Undef
        }
        // Load/store multiple
        else if instr & 0x0800_0000 == 0x0800_0000 {
            Instr::TransferMult {
                load: instr & b!(20) != 0,
                update_base: instr & b!(21) != 0,
                upwards: instr & b!(23) != 0,
                privilige_mode: instr & b!(22) != 0,
                exclude_first: instr & b!(24) != 0,
            }
        }
        // Branch and branch with link
        else if instr & 0x0E00_0000 == 0x0A00_0000 {
            Instr::Branch {
                link: instr & b!(24) != 0,
            }
        }
        // Coprocessor load/store and double register transfers
        else if instr & 0x0E00_0000 == 0x0C00_0000 {
            Instr::CPTransfer {}
        }
        // Coprocessor data processing
        else if instr & 0x0F00_0010 == 0x0E00_0000 {
            Instr::CPDp {}
        }
        // Coprocessor register transfers
        else if instr & 0x0F00_0010 == 0x0E00_0010 {
            Instr::CPRegTransfer {}
        }
        // Software interrupt
        else if instr & 0x0F00_0000 == 0x0F00_0000 {
            Instr::Swi
        }
        // Undefined
        else {
            Instr::Undef
        }
    }

    pub fn decode_uncond(instr: u32) -> Instr {
        if instr & 0x0E00_0000 == 0x0A00_0000 {
            Instr::Blx { imm: true }
        } else {
            Instr::Undef
        }
    }
}
