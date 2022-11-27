use crate::{Core, Interpreter};

pub fn transfer<const ARG: arm_decode::Transfer>(core: &mut Core<Interpreter>, instr: u32) {
    #[allow(non_snake_case)]
    let arm_decode::Transfer {
        load: LOAD,
        add_ofs: ads_ofs,
        ty: TY,
        oper: OPER,
        adr_ty: ADR_TY,
    } = ARG;
}

pub fn misc_transfer<const ARG: arm_decode::MiscTransfer>(
    core: &mut Core<Interpreter>,
    instr: u32,
) {
    #[allow(non_snake_case)]
    let arm_decode::MiscTransfer {
        load: LOAD,
        add_ofs: ADD_OFS,
        imm: IMM,
        ty: TY,
        adr_ty: ADR_TY,
    } = ARG;
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
