use crate::{engines::Engine, Emu};

pub(in crate::engines) fn disassmble_at_adr<E: Engine>(
    emu: &Emu<E>,
    adr: u16,
) -> common_core::disassembly::DisassembledOutput {
    //self.bus.
    todo!()
}
