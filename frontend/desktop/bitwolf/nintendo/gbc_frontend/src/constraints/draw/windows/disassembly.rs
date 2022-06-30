use crate::GBC;
use common_frontend::subwindows::disassembler;

impl disassembler::DisassemblyHook for GBC {
    fn pc_value(&self) -> u64 {
        self.state.reg_file.pc as u64
    }

    fn disassemle_adr(&self, adr: u64) -> common_core::disassembly::DisassembledOutput {
        common_core::disassembly::DisassembledOutput::Instr {
            string_repr: "".to_string(),
            byte_repr: vec![],
            comment: None,
        }
    }

    fn is_breakpoint(&self, adr: u64) -> bool {
        false
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
