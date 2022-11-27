use crate::{
    debug_views::{GlobalState, StaticDV, Ui},
    gui::window::Window,
};
use imgui::{Io, TableFlags};

enum Mode {
    ARM9,
    ARM7,
}

impl Default for Mode {
    fn default() -> Self {
        Self::ARM9
    }
}

#[derive(Default)]
pub struct Registers {
    mode: Mode,
}

impl StaticDV for Registers {
    type Emu = ();

    #[inline]
    fn draw(&mut self, global_state: &GlobalState, _window: &mut Window, ui: &Ui, _io: &Io) {
        ui.menu_bar(|| {
            if ui.button("ARM9") {}
            if ui.button("ARM7") {
                unimplemented!()
            }
        });
        let pc = global_state.registers.pc();
        ui.text(format!("pc: {pc:08X}"));
        {
            let _table = ui.begin_table_with_flags(
                "reg_table",
                4,
                TableFlags::BORDERS | TableFlags::SIZING_STRETCH_SAME,
            );
            for i in 0..16 {
                ui.table_next_column();
                let val = global_state.registers.gpr()[i];
                let reg_name = format!("r{i}");
                let reg_name = if reg_name.len() == 2 {
                    format!("{reg_name}: ")
                } else {
                    format!("{reg_name}:")
                };
                ui.text(format!("{reg_name} {val:08X}"));
            }
        }
    }

    #[inline]
    fn has_menu_bar(&self) -> bool {
        true
    }

    #[inline]
    fn emu_update(&mut self) -> Option<Self::Emu> {
        None
    }
}
