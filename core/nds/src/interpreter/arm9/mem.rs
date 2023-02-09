use crate::cpu::arm9::bus;
use crate::{Core, Interpreter};

pub fn transfer<const ARG: arm_decode::Transfer>(core: &mut Core<Interpreter>, instr: u32) {
    #[allow(non_snake_case)]
    let arm_decode::Transfer {
        load: LOAD,
        add_ofs: ADD_OFS,
        ty: TY,
        oper: OPER,
        adr_ty: ADR_TY,
    } = ARG;
    let rdi = (instr >> 12) as usize & 0xF;
    let rni = (instr >> 16) as usize & 0xF;
    let rd = core.arm9.gpr(rdi);
    let rn = core.arm9.gpr(rni);
    let adr = {
        let oper = match OPER {
            arm_decode::TransfOperTy::Imm => instr & 0xFFF,
            arm_decode::TransfOperTy::Reg { shift } => {
                let rm = core.arm9.gpr(instr as usize & 0xF);
                let shift_imm = (instr >> 7) & 0b11111;
                match shift {
                    arm_decode::ShiftTy::Lsl => rm.wrapping_shl(shift_imm),
                    arm_decode::ShiftTy::Lsr => rm.wrapping_shr(shift_imm),
                    arm_decode::ShiftTy::Asr => (rm as i32).wrapping_shr(shift_imm) as u32,
                    arm_decode::ShiftTy::Ror => {
                        if shift_imm == 0 {
                            panic!("RRX!")
                        } else {
                            rm.rotate_right(shift_imm)
                        }
                    }
                }
            }
        };
        match ADR_TY {
            arm_decode::AdrModeTy::Post => {
                let val = if ADD_OFS {
                    rn.wrapping_add(oper)
                } else {
                    rn.wrapping_sub(oper)
                };
                core.arm9.gpr_set(rni, val);
                rn
            }
            arm_decode::AdrModeTy::Pre => {
                let adr = if ADD_OFS {
                    rn.wrapping_add(oper)
                } else {
                    rn.wrapping_sub(oper)
                };
                core.arm9.gpr_set(rni, adr);
                adr
            }
            arm_decode::AdrModeTy::Offset => {
                if ADD_OFS {
                    rn.wrapping_add(oper)
                } else {
                    rn.wrapping_sub(oper)
                }
            }
        }
    };
    match TY {
        arm_decode::TransfTy::Byte => {
            if LOAD {
                let read = bus::read8(core, adr);
                core.arm9.gpr_set(rdi, read as u32);
            } else {
                bus::write8(core, adr, rd as u8);
            }
        }
        arm_decode::TransfTy::Word => {
            if LOAD {
                let read = bus::read32(core, adr);
                let val = match adr & 0b11 {
                    0b00 => read,
                    0b01 => read.rotate_right(8),
                    0b10 => read.rotate_right(16),
                    0b11 => read.rotate_right(24),
                    _ => unreachable!(),
                };
                core.arm9.gpr_set(rdi, val);
            } else {
                bus::write32(core, adr, rd);
            }
        }
    }
}

pub fn misc_transfer<const ARG: arm_decode::MiscTransfer>(
    core: &mut Core<Interpreter>,
    instr: u32,
) {
    #[allow(non_snake_case)]
    let arm_decode::MiscTransfer {
        add_ofs: ADD_OFS,
        imm: IMM,
        ty: TY,
        adr_ty: ADR_TY,
    } = ARG;
    let rdi = (instr >> 12) as usize & 0xF;
    let rni = (instr >> 16) as usize & 0xF;
    let rn = core.arm9.gpr(rni);
    let rd = core.arm9.gpr(rdi);
    let adr = {
        let oper = if IMM {
            let lo = instr & 0xF;
            let hi = (instr >> 8) & 0xF;
            lo | (hi << 4)
        } else {
            core.arm9.gpr(instr as usize & 0xF)
        };
        match ADR_TY {
            arm_decode::AdrModeTy::Post => {
                core.arm9.gpr_set(
                    rni,
                    if ADD_OFS {
                        rn.wrapping_add(oper)
                    } else {
                        rn.wrapping_sub(oper)
                    },
                );
                rn
            }
            arm_decode::AdrModeTy::Pre => {
                let val = if ADD_OFS {
                    rn.wrapping_add(oper)
                } else {
                    rn.wrapping_sub(oper)
                };
                core.arm9.gpr_set(rni, val);
                val
            }
            arm_decode::AdrModeTy::Offset => {
                if ADD_OFS {
                    rn.wrapping_add(oper)
                } else {
                    rn.wrapping_sub(oper)
                }
            }
        }
    };
    match TY {
        arm_decode::MiscTransfTy::SH => {
            if adr & 0b1 != 0 {
                super::misc::unpred(core, instr);
            }
            let read = bus::read16(core, adr) as i16 as i32 as u32;
            core.arm9.gpr_set(rdi, read);
        }
        arm_decode::MiscTransfTy::H { load } => {
            if adr & 0b1 != 0 {
                super::misc::unpred(core, instr);
            }
            if load {
                let read = bus::read16(core, adr) as u32;
                core.arm9.gpr_set(rdi, read);
            } else {
                bus::write16(core, adr, rd as u16);
            }
        }
        arm_decode::MiscTransfTy::SB => {
            let read = bus::read8(core, adr) as i8 as i32 as u32;
            core.arm9.gpr_set(rdi, read);
        }
    }
}

pub fn transfer_multiple<const ARG: arm_decode::TransferMult>(
    core: &mut Core<Interpreter>,
    instr: u32,
) {
    #[allow(non_snake_case)]
    let arm_decode::TransferMult {
        load: LOAD,
        base_update: BASE_UPDATE,
        adr_ty: ADR_TY,
        privilige_mode: PRIVILIGE_MODE,
    } = ARG;
}

pub fn transfer_double<const ARG: arm_decode::TransferDouble>(
    core: &mut Core<Interpreter>,
    instr: u32,
) {
    #[allow(non_snake_case)]
    let arm_decode::TransferDouble {
        store: STORE,
        add_ofs: ADD_OFS,
        imm: IMM,
        adr_ty: ADR_TY,
    } = ARG;
}

pub fn cp_transfer<const ARG: arm_decode::CPTransfer>(_core: &mut Core<Interpreter>, _instr: u32) {
    unimplemented!()
}
