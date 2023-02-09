use std::path::Path;

use crate::core::Backend;

use super::GUIState;
use imgui::Ui;

pub(super) fn main_bar(state: &mut GUIState, ui: &Ui) {
    ui.main_menu_bar(|| {
        ui.menu("File", || {
            if ui.button("tmp!") {
                state.spawn_core_with_config(
                    Backend::NDS,
                    Path::new("/home/nibble/Downloads/roms/NDS/armwrestler.nds"),
                )
            }
        });
        if !state.core.empty() {
            ui.menu("Debug", || {
                ui.checkbox("CPU", &mut state.debug.cpu_view.opened);
            });
        }
    });
}
