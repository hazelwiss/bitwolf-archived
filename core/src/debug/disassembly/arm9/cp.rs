use super::common::*;
use arm_decode::*;

pub fn qarith<const ARG: QArith>(_: u32, instr: u32) -> String {
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
    let cond = cond_extract(instr);
    format!(
        "{opc}{cond} {rd}, {}",
        if rd_index != rm_index {
            format!("{rm}, {rn}")
        } else {
            rn
        }
    )
}

pub fn dsp_mul<const ARG: DspMul>(_: u32, instr: u32) -> String {
    let arg_ty = #[inline(always)]
    |b: bool| {
        if b {
            "T"
        } else {
            "B"
        }
    };
    let y = arg_ty(ARG.y);
    let cond = cond_extract(instr);
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

pub fn cdp(_: u32, instr: u32) -> String {
    let cond_val = (instr >> 28) & 0xF;
    let cond = cond_from_nibble(cond_val & 0xF);
    let crn = (instr >> 16) & 0xF;
    let crd = (instr >> 12) & 0xF;
    let crm = instr & 0xF;
    let opcode_1 = (instr >> 20) & 0xF;
    let opcode_2 = (instr >> 5) & 0b111;
    let proc = (instr >> 8) & 0xF;
    if cond_val == 0xF {
        format!("cdp2 p{proc}, {opcode_1}, c{crd}, c{crn}, c{crm}, {opcode_2}")
    } else {
        format!("cdp{cond} p{proc}, {opcode_1}, c{crd}, c{crn}, c{crm}, {opcode_2}")
    }
}

pub fn cp_mov<const ARG: CPMov>(_: u32, instr: u32) -> String {
    let crn = (instr >> 16) & 0xF;
    let crm = instr & 0xF;
    let rd = reg((instr >> 12) & 0xF);
    let opcode_1 = (instr >> 21) & 0b111;
    let opcode_2 = (instr >> 5) & 0b111;
    let proc = (instr >> 8) & 0xF;
    let cond_val = (instr >> 28) & 0xF;
    let if_cond = cond_val < 0xF;
    let cond = cond_from_nibble(cond_val & 0xF);
    format!(
        "{} p{proc}, {opcode_1}, {rd}, c{crn}, c{crm}, {opcode_2}",
        if ARG.arm_reg_load {
            if if_cond {
                format!("mrc{cond}")
            } else {
                "mrc2".to_string()
            }
        } else if if_cond {
            format!("mcr{cond}")
        } else {
            "mcr2".to_string()
        },
    )
}

pub fn cp_transfer<const ARG: CPTransfer>(_: u32, instr: u32) -> String {
    let rn = reg((instr >> 16) & 0xF);
    let crd = (instr >> 12) & 0xF;
    let proc = (instr >> 8) & 0xF;
    let imm = instr & 0xFF;
    let option = imm;
    let offset = imm << 2;
    let offset = if ARG.add_ofs {
        format!("0x{offset:X}")
    } else {
        format!("-0x{offset:X}")
    };
    let l = if ARG.n { "l" } else { "" };
    let cond_value = (instr >> 28) & 0xF;
    let if_cond = cond_value < 0xF;
    let cond = cond_from_nibble(instr & 0xF);
    format!(
        "{}{l} p{proc}, c{crd}, {}",
        if ARG.load {
            if if_cond {
                format!("ldc{cond}")
            } else {
                "ldc2".to_string()
            }
        } else if if_cond {
            format!("stc2{cond}")
        } else {
            "stc2".to_string()
        },
        match ARG.adr_ty {
            TransfCpAdrTy::Post => format!("[{rn}], {offset}"),
            TransfCpAdrTy::Unindexed => format!("[{rn}], 0x{option:08X}"),
            TransfCpAdrTy::Pre => format!("[{rn}, {offset}]!"),
            TransfCpAdrTy::Offset => format!("[{rn}, {offset}]"),
        }
    )
}
