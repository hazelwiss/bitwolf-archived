use super::reg_print;
use alloc::string::{String, ToString};
use arm_decode::*;

pub fn dp<const SET_FLAGS: bool, const OPCODE: DpOpcTy, const OPERAND: DpOperTy>(
    instr: u32,
) -> String {
    let s = if SET_FLAGS { "s" } else { "" };
    let rd_i = (instr >> 12) & 0xF;
    let rn_i = (instr >> 16) & 0xF;
    let rd = reg_print(rd_i);
    let rn = reg_print(rn_i);
    let regs = if rd_i == rn_i {
        format!("{rd}")
    } else {
        format!("{rd}, {rn}")
    };
    let ((operand, mvn_operand), mvn) = match OPERAND {
        DpOperTy::Imm => {
            let rotate = instr >> 8 & 0xF;
            let imm = instr & 0xFF;
            let operand = format!("{}", imm.rotate_left(rotate * 2));
            ((operand.clone(), operand), "mvn")
        }
        DpOperTy::Shft { is_reg, ty } => {
            let rm = reg_print(instr & 0xF);
            let shift_print = match ty {
                ShiftTy::Lsl => "lsl",
                ShiftTy::Lsr => "lsr",
                ShiftTy::Asr => "asr",
                ShiftTy::Ror => "ror",
            };
            (
                if is_reg {
                    let rs = reg_print((instr >> 8) & 0xF);
                    (format!("{rm}, {shift_print} {rs}"), format!("{rm}, {rs}"))
                } else {
                    let imm = (instr >> 7) & 0x1F;
                    (format!("{rm}, {shift_print} {imm}"), format!("{rm}, {imm}"))
                },
                shift_print,
            )
        }
    };
    match OPCODE {
        DpOpcTy::And => format!("and{s} {regs}, {operand}"),
        DpOpcTy::Eor => format!("eor{s} {regs}, {operand}"),
        DpOpcTy::Sub => format!("sub{s} {regs}, {operand}"),
        DpOpcTy::Rsb => format!("rsb{s} {regs}, {operand}"),
        DpOpcTy::Add => format!("add{s} {regs}, {operand}"),
        DpOpcTy::Adc => format!("adc{s} {regs}, {operand}"),
        DpOpcTy::Sbc => format!("sbc{s} {regs}, {operand}"),
        DpOpcTy::Rsc => format!("rsc{s} {regs}, {operand}"),
        DpOpcTy::Tst => format!("tst {regs}, {operand}"),
        DpOpcTy::Teq => format!("teq {regs}, {operand}"),
        DpOpcTy::Cmp => format!("cmp {regs}, {operand}"),
        DpOpcTy::Cmn => format!("cmn {regs}, {operand}"),
        DpOpcTy::Orr => format!("orr{s} {regs}, {operand}"),
        DpOpcTy::Mov => format!("mov{s} {rd}, {operand}"),
        DpOpcTy::Bic => format!("bic{s} {regs}, {operand}"),
        DpOpcTy::Mvn => format!("{mvn}{s} {regs}, {mvn_operand}"),
    }
}

pub fn msr(_: u32) -> String {
    format!("msr")
}

pub fn mrs(_: u32) -> String {
    format!("mrs")
}

pub fn mul(_: u32) -> String {
    format!("mul")
}

pub fn dsp_mul(_: u32) -> String {
    format!("dsp mul")
}
