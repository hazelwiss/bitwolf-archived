use crate::window_loop::ImguiCtx;

pub(crate) struct DisasmChildWindowBuilder {
    pub instructions: (),
}

impl DisasmChildWindowBuilder {
    pub fn build(self, imgui_ctx: &ImguiCtx) {
        let ui = imgui_ctx.ui();
        imgui::ChildWindow::new("Disassembler").build(ui, || {
            //for instr in self.instructions.iter() {
            //    todo!()
            //}
        });
    }
}
