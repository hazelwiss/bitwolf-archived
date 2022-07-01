use super::Emu;
use crate::state::substates;
use common_core::disassembly::DisassembledOutput;
use gbc_backend::engines::interpreter::{self, Interpreter};

pub(super) fn get(emu: &mut Emu<Interpreter>) -> substates::Disassembly {
    let mut vec = vec![];
    let mut adr = 0;
    while adr < 0x8000 {
        let disasm = interpreter::debug::disassemble::disassemble(emu, adr);
        match &disasm.output {
            DisassembledOutput::Instr { byte_repr, .. } => adr += byte_repr.len() as u16,
            DisassembledOutput::Data { .. } => adr += 1,
        }
        vec.push(disasm.output);
    }
    substates::Disassembly { rom: vec }
}
