use super::common::*;
use arm_decode::*;

pub fn dp<const ARG: Dp>(_: u32, instr: u32) -> String {
    let rd_i = (instr >> 12) & 0xF;
    let rn_i = (instr >> 16) & 0xF;
    let rd = reg(rd_i);
    let rn = reg(rn_i);
    let regs = if rd_i == rn_i {
        rd.to_string()
    } else {
        format!("{rd}, {rn}")
    };
    let ((operand, mov_oper), mov) = match ARG.oper {
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
                            rm.clone()
                        },
                        if imm != 0 {
                            format!("{rm}, #0x{imm:X}")
                        } else {
                            rm
                        },
                    )
                },
                shift_print,
            )
        }
    };
    let s = if ARG.flags { "s" } else { "" };
    let cond = cond_extract(instr);
    match ARG.opc {
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

pub fn clz(_: u32, instr: u32) -> String {
    let rd = reg((instr >> 12) & 0xF);
    let rm = reg(instr & 0xF);
    let cond = cond_extract(instr);
    format!("clz{cond} {rd}, {rm}")
}

pub fn msr<const ARG: Msr>(_: u32, instr: u32) -> String {
    let psr = if ARG.r { "spsr" } else { "cpsr" };
    let arg = if ARG.imm {
        let imm = instr & 0xF;
        let rot = (instr >> 8) & 0xF;
        let val = imm.rotate_right(rot * 2);
        format!("0x{val:X}")
    } else {
        reg(instr & 0xF)
    };
    let cond = cond_extract(instr);
    format!("msr{cond} {psr}_, {arg}")
}

pub fn mrs<const ARG: Mrs>(_: u32, instr: u32) -> String {
    let rd = reg((instr >> 12) & 0xF);
    let cond = cond_extract(instr);
    format!("mrs{cond} {rd}, {}", if ARG.r { "spsr" } else { "cpsr" })
}

pub fn mul<const ARG: Mul>(_: u32, instr: u32) -> String {
    let rm = reg((instr >> 8) & 0xF);
    let rs = reg((instr >> 8) & 0xF);
    let rd = reg((instr >> 16) & 0xF);
    let rn = reg((instr >> 12) & 0xF);
    let rdhi = reg((instr >> 16) & 0xF);
    let rdlo = reg((instr >> 12) & 0xF);
    let flags = if ARG.flags { "s" } else { "" };
    let cond = cond_extract(instr);
    match ARG.ty {
        MulTy::Mla => format!("mla{cond}{flags} {rd}, {rm}, {rs}, {rn}"),
        MulTy::Mul => format!("mul{cond}{flags} {rd}, {rm}, {rs}"),
        MulTy::Smlal => format!("smlal{cond}{flags} {rdlo}, {rdhi}, {rm}, {rs}"),
        MulTy::Smull => format!("smull{cond}{flags} {rdlo}, {rdhi}, {rm}, {rs}"),
        MulTy::Umlal => format!("umlal{cond}{flags} {rdlo}, {rdhi}, {rm}, {rs}"),
        MulTy::Umull => format!("umull{cond}{flags} {rdlo}, {rdhi}, {rm}, {rs}"),
    }
}
