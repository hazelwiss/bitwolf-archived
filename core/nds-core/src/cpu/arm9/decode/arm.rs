use core::mem::transmute;

use bitmatch_proc::bitmatch;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[derive(Clone, Copy)]
pub enum DpShiftOp {
    Reg,
    Imm,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum DpOp {
    And = 0,
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

pub enum TransferOp {
    WordOrByte,
    HalfwordOrSByte { s: bool, h: bool },
}

pub enum Instr {
    Dp {
        i: bool,
        op: DpOp,
        s: bool,
    },
    Msr,
    Mrs,
    Bx,
    Clz,
    Blx,
    SatAddSub,
    DspMul,
    Swi,
    Mul {
        accumulate: bool,
    },
    Mull {
        accumulate: bool,
    },
    Transfer {
        i: bool,
        p: bool,
        u: bool,
        b: bool,
        w: bool,
        l: bool,
        op: TransferOp,
        load: bool,
    },
    Swap,
    Undef,
}

pub const fn decode_cond(instr: u32) -> Instr {
    let instr = instr & 0x0FFFFFFF;
    // Miscellaneous instructions (3-3)
    if instr & 0x0F90_0010 == 0x0100_0000 || instr & 0x0F90_0090 == 0x0100_0010 {
        // Some miscelanous instructions must be handled before DP instructions due
        // to a 'hole' within the encoding table caused by the opcode field being equal
        // to 0b10xx while S is zero.
        let oper = (instr >> 4) & 0xF;
        let op = (instr >> 21) & 0b11;
        bitmatch!(match oper {
            "0000" => {
                let r = op >> 1 != 0;
                if op & 1 == 0 {
                    Instr::Msr
                } else {
                    Instr::Mrs
                }
            }
            "0001" if op != 0 => {
                if op == 0b01 {
                    Instr::Bx
                } else {
                    Instr::Clz
                }
            }
            "0011" if op == 0b01 => {
                Instr::Blx
            }
            "0101" => {
                Instr::SatAddSub
            }
            "0111" if op == 0b01 => {
                Instr::Swi
            }
            "1yx0" => {
                Instr::DspMul
            }
            _ => Instr::Undef,
        })
    }
    // Data processing immediate shift
    else if instr & 0x0E00_0010 == 0x0000_0000 {
        Instr::Dp {
            i: true,
            op: unsafe { transmute((instr >> 21) & 0xF) },
            s: (instr >> 20) != 0,
        }
    }
    // Data processing register shift
    else if instr & 0x0E00_0090 == 0x0000_0010 {
        Instr::Dp {
            i: false,
            op: unsafe { transmute((instr >> 21) & 0xF) },
            s: (instr >> 20) != 0,
        }
    }
    // Multiples, extra load/stores
    else if instr & 0x0E00_0090 == 0x0000_0090 {
        let oper = (instr >> 5) & 0b11;
        let p = (instr >> 24) & 1 != 0;
        let u = (instr >> 24) & 1 != 0;
        bitmatch!(match oper {
            "00" => {
                let bits = (instr >> 20) & 0x1F;
                bitmatch!(match bits {
                    "000as" => Instr::Mul { accumulate: true },
                    "01uas" => Instr::Mull { accumulate: true },
                    "10b00" => Instr::Swap,
                    _ => Instr::Undef,
                })
            }
            "01" => {
                let bits = (instr >> 20) & 0b111;
                bitmatch!(match bits {
                    "0wl" => todo!(),
                    "1wl" => todo!(),
                    _ => Instr::Undef,
                })
            }
            "1x" => {
                let bits = (instr >> 20) & 0b111;
                bitmatch!(match bits {
                    _ => Instr::Undef,
                })
            }
            _ => Instr::Undef,
        })
    }
    // Data processing immediate
    else if instr & 0x0E00_0000 == 0x0100_0000 {
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

pub fn decode_uncond(instr: u32) -> Instr {
    todo!()
}

pub fn decode(instr: u32) -> Instr {
    let cond = instr >> 28;
    if cond == 0b1111 {
        decode_uncond(instr)
    } else {
        decode_uncond(instr)
    }
}
