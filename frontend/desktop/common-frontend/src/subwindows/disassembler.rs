#![allow(unused)]
#![allow(unreachable_code)]

use common_core::disassembly::DisassembledOutput;
use imgui::gui::TableFlags;

pub trait DisassemblyHook {
    fn pc_value(&self) -> u64;

    fn is_breakpoint(&self, adr: u64) -> bool;

    fn get_start_adr(&self) -> u64;

    fn get_disassembly_output(&self) -> &Vec<DisassembledOutput>;
}

pub fn disasm_subwindow(
    hook: &mut impl DisassemblyHook,
    draw_ctx: &imgui::DrawContext,
    subwindow: imgui::gui::ChildWindow,
) {
    return;
    let ui = draw_ctx.ui();
    let mut adr = hook.get_start_adr();
    let pc = hook.pc_value();
    let output = hook.get_disassembly_output();
    subwindow.build(ui, || {
        let _table = ui.begin_table_with_flags(
            "disasm_table",
            5,
            TableFlags::BORDERS_OUTER
                | TableFlags::BORDERS_INNER_V
                | TableFlags::SIZING_STRETCH_PROP,
        );
        ui.table_next_row();
        for line in output {
            ui.table_next_column();
            if hook.is_breakpoint(adr) {
                ui.text("B");
            }
            ui.table_next_column();
            ui.text(format!("{adr:08X}"));
            ui.table_next_column();
            match line {
                DisassembledOutput::Instr {
                    string_repr,
                    byte_repr,
                    comment,
                } => {
                    ui.text(string_repr);
                    ui.table_next_column();
                    let mut string = format!("{:02X}", byte_repr[0]);
                    for i in 1..byte_repr.len() {
                        let byte = byte_repr[i];
                        string.insert_str(string.len(), &format!(", {byte:02X}"));
                    }
                    string = format!("[{string}]");
                    ui.text(string);
                    ui.table_next_column();
                    if let Some(comment) = comment {
                        ui.text(format!("// {comment}"));
                    }
                    adr += byte_repr.len() as u64;
                }
                DisassembledOutput::Data { data } => {
                    ui.text(format!("{data:02X}"));
                    ui.table_next_row();
                    adr += 1;
                }
            }
        }
    });
}
