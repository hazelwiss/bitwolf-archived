use crate::GBC;
use common_frontend::subwindows::disassembler;

impl disassembler::DisassemblyHook for GBC {
    fn pc_value(&self) -> u64 {
        self.com.state.reg_file.pc as u64
    }

    fn is_breakpoint(&self, adr: u64) -> bool {
        self.com.state.ctrl.breakpoints.contains(&(adr as u16))
    }

    fn get_start_adr(&self) -> u64 {
        0x00
    }

    fn get_disassembly_output(&self) -> &Vec<common_core::disassembly::DisassembledOutput> {
        &self.com.state.disasm.rom
    }
}

pub fn draw(gbc: &mut GBC, draw_ctx: &mut imgui::DrawContext) {
    let ui = draw_ctx.ui();
    imgui::gui::Window::new("Disassembly").build(ui, || {
        disassembler::disasm_subwindow(
            gbc,
            draw_ctx,
            imgui::gui::ChildWindow::new("disasm")
                .always_auto_resize(false)
                .size(ui.content_region_avail()),
        );
    });
}
