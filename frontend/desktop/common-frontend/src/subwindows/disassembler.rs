pub trait DisassemblyHook {
    fn pc_value(&self) -> u64;

    fn disassemle_adr(&self, adr: u64) -> common_core::disassembly::DisassembledOutput;

    fn is_breakpoint(&self, adr: u64) -> bool;
}

pub fn disasm_subwindow(
    hook: &mut impl DisassemblyHook,
    draw_ctx: &mut imgui::DrawContext,
    subwindow: imgui::gui::ChildWindow,
) {
    let ui = draw_ctx.ui();
    subwindow.build(ui, || {
        ui.text("TODO!");
    });
}
