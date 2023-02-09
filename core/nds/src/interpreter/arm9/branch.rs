use crate::{Core, Interpreter};

pub fn blx<const IMM: bool>(core: &mut Core<Interpreter>, instr: u32) {
    unimplemented!()
}

pub fn bx(core: &mut Core<Interpreter>, instr: u32) {
    let rm = instr & 0xF;
    if rm & 0b11 == 0b10 {
        super::misc::unpred(core, instr);
    }
    if rm & 0b1 != 0 {
        panic!("cannot switch to thumb! unimplemented!")
    } else {
        core.arm9.pc_set(rm & !0b11);
    }
}

pub fn b<const ARG: arm_decode::B>(core: &mut Core<Interpreter>, instr: u32) {
    #[allow(non_snake_case)]
    let arm_decode::B { link: LINK } = ARG;
    let imm = (((instr & 0x00FFFFFF) as i32) << 8) >> 6;
    let pc = core.arm9.pc();
    let new_pc = pc.wrapping_add_signed(imm);
    if LINK {
        core.arm9.lr_set(core.arm9.pc());
    }
    core.arm9.pc_set(new_pc);
}
