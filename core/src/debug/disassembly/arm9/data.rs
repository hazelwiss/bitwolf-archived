use super::{cond, reg};
use alloc::string::String;
use arm_decode::*;

pub fn dp<const ARG: arm_decode::Dp>(instr: u32) -> String {
    let s = if ARG.set_flags { "s" } else { "" };
    let rd_i = (instr >> 12) & 0xF;
    let rn_i = (instr >> 16) & 0xF;
    let rd = reg(rd_i);
    let rn = reg(rn_i);
    let regs = if rd_i == rn_i {
        format!("{rd}")
    } else {
        format!("{rd}, {rn}")
    };
    let ((operand, mov_oper), mov) = match ARG.operand {
        DpOperTy::Imm => {
            let rotate = (instr >> 8) & 0xF;
            let imm = instr & 0xFF;
            let operand = format!("#0x{:X}", imm.rotate_right(rotate * 2));
            ((operand.clone(), operand), "mov")
        }
        DpOperTy::Shft { is_reg, ty } => {
            let rm = reg(instr & 0xF);
            let shift_print = match ty {
                ShiftTy::Lsl => "lsl",
                ShiftTy::Lsr => "lsr",
                ShiftTy::Asr => "asr",
                ShiftTy::Ror => "ror",
            };
            (
                if is_reg {
                    let rs = reg((instr >> 8) & 0xF);
                    (format!("{rm}, {shift_print} {rs}"), format!("{rm}, {rs}"))
                } else {
                    let imm = (instr >> 7) & 0x1F;
                    (
                        if imm != 0 {
                            format!("{rm}, {shift_print} #0x{imm:X}")
                        } else {
                            format!("{rm}")
                        },
                        if imm != 0 {
                            format!("{rm}, #0x{imm:X}")
                        } else {
                            format!("{rm}")
                        },
                    )
                },
                shift_print,
            )
        }
    };
    let cond = cond(instr);
    match ARG.opcode {
        DpOpcTy::And => format!("and{cond}{s} {regs}, {operand}"),
        DpOpcTy::Eor => format!("eor{cond}{s} {regs}, {operand}"),
        DpOpcTy::Sub => format!("sub{cond}{s} {regs}, {operand}"),
        DpOpcTy::Rsb => format!("rsb{cond}{s} {regs}, {operand}"),
        DpOpcTy::Add => format!("add{cond}{s} {regs}, {operand}"),
        DpOpcTy::Adc => format!("adc{cond}{s} {regs}, {operand}"),
        DpOpcTy::Sbc => format!("sbc{cond}{s} {regs}, {operand}"),
        DpOpcTy::Rsc => format!("rsc{cond}{s} {regs}, {operand}"),
        DpOpcTy::Tst => format!("tst{cond} {regs}, {operand}"),
        DpOpcTy::Teq => format!("teq{cond} {regs}, {operand}"),
        DpOpcTy::Cmp => format!("cmp{cond} {regs}, {operand}"),
        DpOpcTy::Cmn => format!("cmn{cond} {regs}, {operand}"),
        DpOpcTy::Orr => format!("orr{cond}{s} {regs}, {operand}"),
        DpOpcTy::Bic => format!("bic{cond}{s} {regs}, {operand}"),
        DpOpcTy::Mvn => format!("mvn{cond}{s} {regs}, {operand}"),
        DpOpcTy::Mov => format!("{mov}{cond}{s} {rd}, {mov_oper}"),
    }
}

pub fn clz(instr: u32) -> String {
    let rd = reg((instr >> 12) & 0xF);
    let rm = reg(instr & 0xF);
    let cond = cond(instr);
    format!("clz{cond} {rd}, {rm}")
}

pub fn msr<const ARG: arm_decode::Msr>(instr: u32) -> String {
    let psr = if ARG.r { "spsr" } else { "cpsr" };
    let arg = if ARG.imm {
        let imm = instr & 0xF;
        let rot = (instr >> 8) & 0xF;
        let val = imm.rotate_right(rot * 2);
        format!("0x{val:X}")
    } else {
        reg(instr & 0xF)
    };
    let cond = cond(instr);
    format!("msr{cond} {psr}_, {arg}")
}

pub fn mrs<const ARG: arm_decode::Mrs>(instr: u32) -> String {
    let rd = reg((instr >> 12) & 0xF);
    let cond = cond(instr);
    format!("mrs{cond} {rd}, {}", if ARG.r { "spsr" } else { "cpsr" })
}

pub fn mul<const ARG: arm_decode::Mul>(instr: u32) -> String {
    let cond = cond(instr);
    let rd_index = (instr >> 16) & 0xF;
    let rm_index = instr & 0xF;
    let rd = reg(rd_index);
    let rm = reg(rm_index);
    let rs = reg((instr >> 8) & 0xF);
    format!(
        "mul{cond}{} {rd}, {}",
        if ARG.set_flags { "s" } else { "" },
        if rd_index != rm_index {
            format!("{rm}, {rs}")
        } else {
            rs
        }
    )
}

pub fn qarith<const ARG: arm_decode::QArith>(instr: u32) -> String {
    let opc_m = (ARG.sub, ARG.doubles);
    let opc = match opc_m {
        (true, true) => "qdsub",
        (true, false) => "qsub",
        (false, true) => "qdadd",
        (false, false) => "qadd",
    };
    let rd_index = (instr >> 12) & 0xF;
    let rm_index = instr & 0xF;
    let rd = reg(rd_index);
    let rm = reg(rm_index);
    let rn = reg((instr >> 16) & 0xF);
    let cond = cond(instr);
    format!(
        "{opc}{cond} {rd}, {}",
        if rd_index != rm_index {
            format!("{rm}, {rn}")
        } else {
            rn
        }
    )
}

pub fn dsp_mul<const ARG: arm_decode::DspMul>(_: u32) -> String {
    format!("dsp mul")
}
