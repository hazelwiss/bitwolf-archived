use crate::core::Core___;
use crate::gui::ProgramState;
use imgui::Ui;
use std::path::Path;

pub(super) fn main_bar<C: Core___>(state: &mut ProgramState<C>, ui: &Ui) {
    ui.main_menu_bar(|| {
        ui.menu("File", || {
            //if ui.button("tmp!") {
            //    state.spawn_core_with_config(
            //        Backend::NDS,
            //        Path::new("/home/nibble/Downloads/roms/NDS/armwrestler.nds"),
            //    )
            //}
        });
        //if !state.core.empty() {
        //    ui.menu("Debug", || {
        //        ui.checkbox("CPU", &mut state.debug.cpu_view.opened);
        //    });
        //}
    });
}
