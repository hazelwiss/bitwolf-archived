pub fn disassemble(
    emu: &Emu<Interpreter>,
    adr: u16,
) -> common_core::disassembly::DisassembledOutput {
    debug::disassembly::disassmble_at_adr(emu, adr)
}
