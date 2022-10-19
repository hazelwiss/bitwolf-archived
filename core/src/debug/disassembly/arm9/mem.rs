use super::{cond, reg};
use alloc::string::String;
use arm_decode::*;

#[inline]
fn transf_oper(
    instr: u32,
    base_reg: u32,
    is_offset_added: bool,
    operand: TransfOperTy,
    addressing: TransfAdrTy,
) -> String {
    let sign = if is_offset_added { "" } else { "-" };
    let operand = match operand {
        TransfOperTy::Imm => {
            let imm = instr & 0xFFF;
            format!(", {sign}#0x{imm:X}")
        }
        TransfOperTy::Reg { shift } => {
            let rm = reg(instr & 0xF);
            let shift_imm = (instr >> 7) & 0x1F;
            match shift {
                ShiftTy::Lsl => {
                    if shift_imm == 0 {
                        format!("")
                    } else {
                        format!(", {sign}{rm}, lsl {shift_imm:X}")
                    }
                }
                ShiftTy::Lsr => format!(", {sign}{rm}, lsr {shift_imm:X}"),
                ShiftTy::Asr => format!(", {sign}{rm}, asr {shift_imm:X}"),
                ShiftTy::Ror => format!(", {sign}{rm}, ror {shift_imm:X}"),
            }
        }
    };
    let base = reg(base_reg & 0xF);
    match addressing {
        TransfAdrTy::Post => format!("[{base}]{operand}"),
        TransfAdrTy::Pre => format!("[{base}{operand}]!"),
        TransfAdrTy::Offset => format!("[{base}{operand}]"),
    }
}

pub fn transfer<const ARG: arm_decode::Transfer>(instr: u32) -> String {
    let cond = cond(instr);
    let rd = reg((instr >> 12) & 0xF);
    let rn = (instr >> 16) & 0xF;
    let operand = transf_oper(
        instr,
        rn,
        ARG.is_ofs_add_or_sub,
        ARG.operand,
        ARG.addressing,
    );
    format!(
        "{}{}{cond}{} {rd}, {operand}",
        if ARG.load { "ldr" } else { "str" },
        if ARG.byte { "b" } else { "" },
        "" // TODO: should be able to be "t" or ""
    )
}

pub fn misc_transfer<const ARG: arm_decode::MiscTransfer>(_: u32) -> String {
    format!("misc_transfer")
}

pub fn transfer_multiple<const ARG: arm_decode::TransferMult>(_: u32) -> String {
    format!("transfer multiple")
}

pub fn transfer_double<const ARG: arm_decode::TransferDouble>(_: u32) -> String {
    format!("transfer double")
}
