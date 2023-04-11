use super::State;
use crate::frontend::Frontend;
use imgui::Ui;
use std::path::Path;

pub(super) fn main_bar(state: &mut State, ui: &Ui) {
    ui.main_menu_bar(|| {
        ui.menu("File", || {
            //if ui.button("tmp!") {
            //    state.spawn_core_with_config(
            //        Backend::NDS,
            //        Path::new("/home/nibble/Downloads/roms/NDS/armwrestler.nds"),
            //    )
            //}
        });
        match &mut state.frontend {
            Frontend::None => {}
            other => ui.menu("Debug", || other.draw_menu_debug(ui)),
        }
    });
}
