mod instr;

use crate::{
    cpu::{arm9::bus, bus::CPUAccess},
    Core, Interpreter,
};

impl Core<Interpreter> {
    fn arm9_next_instr_adr(&self) -> u32 {
        self.arm9.registers.get_pc()
    }
}

#[inline]
pub fn step(core: &mut Core<Interpreter>) {
    let instr = bus::read32::<CPUAccess, _>(core, core.arm9.registers.get_pc());
    core.arm9
        .registers
        .set_pc(core.arm9.registers.get_pc().wrapping_add(4));
    if (instr >> 28) & 0xF == 0xF {
        if (instr >> 25) & 0b111 == 0b101 {
            instr::branch::blx::<true>(core, instr)
        } else {
            instr::misc::undef(core, instr);
        }
    } else {
        let index = ((instr >> 4) & 0xF) | ((instr >> 16) & 0xFF0);
        instr::INSTR_CONDITIONAL[index as usize](core, instr)
    }
}

#[inline]
pub fn run(core: &mut Core<Interpreter>, cycles: u64) {
    for _ in 0..cycles {
        step(core)
    }
}
