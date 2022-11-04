use super::common::*;
use arm_decode::*;

#[inline(always)]
fn adr_m_3(instr: u32, rn: u32, add_ofs: bool, imm: bool, adr_ty: TransfAdrTy) -> String {
    let rn = reg(rn & 0xF);
    let mut show_oper = true;
    let oper = format!(
        "{}{}",
        if add_ofs { "" } else { "-" },
        if imm {
            let imm = (instr & 0xF) | ((instr >> 4) & 0xF0);
            show_oper = imm > 0;
            format!("0x{imm:02X}")
        } else {
            reg(instr & 0xF)
        }
    );
    if show_oper {
        match adr_ty {
            TransfAdrTy::Post { .. } => format!("[{rn}], {oper}"),
            TransfAdrTy::Pre => todo!("[{rn}, {oper}]!"),
            TransfAdrTy::Offset => format!("[{rn}, {oper}]"),
        }
    } else {
        format!("[{rn}]")
    }
}

pub fn transfer<const ARG: arm_decode::Transfer>(_: u32, instr: u32) -> String {
    let cond = cond(instr);
    let rd = reg((instr >> 12) & 0xF);
    let rn = (instr >> 16) & 0xF;
    let (operand, transform) = {
        let sign = if ARG.add_ofs { "" } else { "-" };
        let operand = match ARG.oper {
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
                            "".to_string()
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
        let base = reg(rn & 0xF);
        match ARG.adr_ty {
            TransfAdrTy::Post { translation } => (format!("[{base}]{operand}"), translation),
            TransfAdrTy::Pre => (format!("[{base}{operand}]!"), false),
            TransfAdrTy::Offset => (format!("[{base}{operand}]"), false),
        }
    };
    format!(
        "{}{}{cond}{} {rd}, {operand}",
        if ARG.load { "ldr" } else { "str" },
        match ARG.ty {
            TransfTy::Byte => "b",
            TransfTy::Word => "",
        },
        if transform { "t" } else { "" }
    )
}

pub fn misc_transfer<const ARG: arm_decode::MiscTransfer>(_: u32, instr: u32) -> String {
    let rd = reg((instr >> 12) & 0xF);
    let rn = (instr >> 16) & 0xF;
    let oper = adr_m_3(instr, rn, ARG.add_ofs, ARG.imm, ARG.adr_ty);
    let cond = cond(instr);
    format!(
        "{}{cond}{} {rd}, {}",
        if ARG.load { "ldr" } else { "str" },
        match ARG.ty {
            MiscTransfTy::SH => "sh",
            MiscTransfTy::H => "h",
            MiscTransfTy::SB => "sb",
        },
        oper
    )
}

pub fn transfer_multiple<const ARG: arm_decode::TransferMult>(_: u32, instr: u32) -> String {
    let rn = reg((instr >> 16) & 0xF);
    let register_list = {
        let mut len = 0;
        let mut start = 0;
        let mut print = String::new();
        let add_reg_single = #[inline(always)]
        |index: u32, str: &mut String| {
            let reg = reg(index & 0xF);
            str.push_str(&format!("{reg},"));
        };
        let add_reg_multiple = #[inline(always)]
        |start: u32, end: u32, str: &mut String| {
            let reg_lower = reg(start & 0xF);
            let reg_upper = reg((start + end) & 0xF);
            str.push_str(&format!("{reg_lower} - {reg_upper},"));
        };
        for i in 0..16 {
            let active = instr & (1 << i) != 0;
            if active {
                len += 1;
            } else if len == 0 {
                start = i;
            } else if len == 1 {
                add_reg_single(start, &mut print);
            } else {
                add_reg_multiple(start, len + 1, &mut print);
                len = 0;
                start = i;
            }
        }
        if len == 1 {
            add_reg_single(start, &mut print);
        } else if len > 1 {
            add_reg_multiple(start, len + 1, &mut print);
        }
        print.pop();
        format!("{{{print}}}")
    };
    let cond = cond(instr);
    format!(
        "{}{cond}{} {rn}{}, {register_list}",
        if ARG.load { "ldm" } else { "stm" },
        match ARG.adr_ty {
            TransfMulAdrTy::IA => "ia",
            TransfMulAdrTy::IB => "ib",
            TransfMulAdrTy::DA => "da",
            TransfMulAdrTy::DB => "db",
        },
        if ARG.base_update { "!" } else { "" },
    )
}

pub fn transfer_double<const ARG: arm_decode::TransferDouble>(_: u32, instr: u32) -> String {
    let start = (instr >> 12) & 0xE; // ignore lower bit.
    let rn = (instr >> 16) & 0xF;
    let lo = reg(start);
    let hi = reg(start + 1);
    let oper = adr_m_3(instr, rn, ARG.add_ofs, ARG.imm, ARG.adr_ty);
    let cond = cond(instr);
    format!(
        "{}{cond}d, {{{lo},{hi}}}, {oper}",
        if ARG.store { "str" } else { "ldr" }
    )
}
