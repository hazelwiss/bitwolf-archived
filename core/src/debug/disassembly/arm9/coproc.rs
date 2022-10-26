use super::common::*;
use arm_decode::*;

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

pub fn dsp_mul<const ARG: arm_decode::DspMul>(instr: u32) -> String {
    let arg_ty = #[inline(always)]
    |b: bool| {
        if b {
            "T"
        } else {
            "B"
        }
    };
    let y = arg_ty(ARG.y);
    let cond = cond(instr);
    let rd = reg((instr >> 16) & 0xF);
    let rn = reg((instr >> 12) & 0xF);
    let rs = reg((instr >> 8) & 0xF);
    let rm = reg(instr & 0xF);
    match ARG.ty {
        DspMulTy::Smul { x } => format!("smul{}{y}{cond} {rd}, {rm}, {rs}", arg_ty(x)),
        DspMulTy::Smla { x } => format!("smla{}{y}{cond} {rd}, {rm}, {rs}, {rn}", arg_ty(x)),
        DspMulTy::Smulw => format!("smulw{y}{cond} {rd}, {rm}, {rs}"),
        DspMulTy::Smlaw => format!("smlaw{y}{cond} {rd}, {rm}, {rs}, {rn}"),
        DspMulTy::Smlal { x } => format!("smlal{}{y}{cond} {rn}, {rd}, {rm}, {rs}", arg_ty(x)),
    }
}
