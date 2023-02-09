use crate::debug::DebugInterface;
use crate::gui::{self, GUIState};
use imgui::{TableBgTarget, TableFlags, Ui};
use std::fmt::{Display, UpperHex};

pub struct CPUView {
    pub opened: bool,
    arm9_previous_gpr: [u32; 16],
}

impl CPUView {
    pub fn new() -> Self {
        Self {
            opened: false,
            arm9_previous_gpr: [0; 16],
        }
    }
}

pub(super) fn draw(state: &mut GUIState, ui: &Ui) {
    let view = &mut state.debug.cpu_view;
    if view.opened && let Some(core) = state.core.as_trait_mut() {
        let dbg = core.dbg_mut();
        let reg_info = &dbg.reg_info;
        ui.window("CPU").build(|| {
            ui.child_window("GPR").build(||{
                match reg_info{
                    lib::RegisterInfo::NDS { arm9_gpr } => {
                        ui.text("ARM9");
                        draw_gpr_table(ui, "arm9-gpr", 4, arm9_gpr, &view.arm9_previous_gpr);
                        ui.text("ARM7");
                        {
                            ui.text("Placeholder!");
                        }
                        view.arm9_previous_gpr = *arm9_gpr;
                    },
                }
            });
        });
    }
}

fn draw_gpr_table<T: UpperHex + Eq, const SIZE: usize>(
    ui: &Ui,
    table_name: impl Display,
    columns: usize,
    regs: &[T; SIZE],
    prev_regs: &[T; SIZE],
) {
    let _table = ui.begin_table_with_flags(
        table_name.to_string(),
        4,
        TableFlags::BORDERS | TableFlags::SIZING_STRETCH_SAME,
    );
    for (i, gpr) in regs.iter().enumerate() {
        ui.table_next_column();
        if unsafe { prev_regs.get_unchecked(i) } != gpr {
            ui.table_set_bg_color(TableBgTarget::CELL_BG, [0.3, 0.3, 0.6, 1.0]);
        }
        ui.label_text(format!("R{i}"), format!("{gpr:08X}"))
    }
}
