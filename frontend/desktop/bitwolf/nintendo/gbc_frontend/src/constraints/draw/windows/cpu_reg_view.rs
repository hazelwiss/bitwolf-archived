use crate::GBC;
use imgui::gui::TableFlags;

// TODO: Make window stretchable
pub fn draw(gbc: &mut GBC, draw_ctx: &mut imgui::DrawContext) {
    let ui = draw_ctx.ui();
    imgui::gui::Window::new("CPU Register Window")
        .always_auto_resize(true)
        .build(ui, || {
            let ui = draw_ctx.ui();
            let flags = TableFlags::BORDERS | TableFlags::SIZING_STRETCH_SAME;
            {
                let _table = ui.begin_table_with_flags("special registers", 2, flags);
                ui.table_next_column();
                ui.text(&format!("pc: {:04X}", gbc.state.reg_file.pc));
                ui.table_next_column();
                ui.text(&format!("sp: {:04X}", gbc.state.reg_file.sp));
            }
            {
                let _table = ui.begin_table_with_flags("gpr", 2, flags);
                ui.table_next_column();
                ui.text(&format!("af: {:04X}", gbc.state.reg_file.af));
                ui.table_next_column();
                ui.text(&format!("bc: {:04X}", gbc.state.reg_file.bc));
                ui.table_next_column();
                ui.text(&format!("de: {:04X}", gbc.state.reg_file.de));
                ui.table_next_column();
                ui.text(&format!("hl: {:04X}", gbc.state.reg_file.hl));
            }
            ui.text(format!(
                "z[{:1b}] n[{:1b}] h[{:1b}] c[{:1b}]",
                gbc.state.reg_file.z as u8,
                gbc.state.reg_file.n as u8,
                gbc.state.reg_file.h as u8,
                gbc.state.reg_file.c as u8
            ));
        });
}
