use super::nds::{self as frontend, DebugConf};
use nds as core;

#[derive(Default)]
pub struct DebugGlobalState {
    pub arm9_gpr: [u32; 16],
}

impl DebugGlobalState {
    pub fn arm9_pc(&self) -> u32 {
        self.arm9_gpr[15]
    }
}

pub fn interp_update(state: &mut frontend::State) {
    trace!("update");
}

pub fn setup(state: &mut frontend::State) {
    trace!("setup")
}

pub fn core_interp_run(core: &mut core::Core<core::Interpreter>) {
    trace!("interp run");
}

pub fn core_mk_debug_state(
    core: &mut core::Core<core::Interpreter>,
    conf: &DebugConf,
) -> frontend::DebugState {
    use crate::gui::debug::nds::disasm;
    frontend::DebugState {
        global_debug_state: DebugGlobalState {
            arm9_gpr: core.arm9.gpr,
        },
        disasm: disasm::State {
            line: conf.disasm.line,
            instr: {
                let mut vec = vec![];
                for i in 0..conf.disasm.count + 1 {
                    let (instr, print) = nds::debug::disasm_arm9::disassemble_at_adr(
                        core,
                        (i as u32) * 4 + conf.disasm.line,
                    );
                    vec.push(disasm::Instr {
                        bytes: Box::new(instr.to_le_bytes()),
                        print,
                    })
                }
                vec.into_boxed_slice()
            },
        },
    }
}
