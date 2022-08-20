use super::NDSFrontend;
use crate::{common::windows::disassembler::DisasmChildWindowBuilder, window_loop::ImguiCtx};

pub fn update_panels(nds: &mut NDSFrontend, imgui_ctx: &mut ImguiCtx) {
    let ui = imgui_ctx.ui();
    if nds.disassembler_open {
        imgui::Window::new("Disassembler")
            .opened(&mut nds.disassembler_open)
            .build(ui, || {
                DisasmChildWindowBuilder { instructions: () }.build(imgui_ctx);
            });
    }
}

pub fn rom_header_info(nds: &mut NDSFrontend, imgui_ctx: &mut ImguiCtx) {}
