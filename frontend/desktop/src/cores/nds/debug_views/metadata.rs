use crate::common::debug_view::View;
use crate::ui::gfx::{self, GfxContext};
use imgui::sys::ImGuiTableFlags_Borders;
use imgui::Ui;
use nds_core::rom::CartridgeHeader;

macro_rules! tblrow {
    ($ui:ident, $name:ident:$print:expr) => {{
        let ui = $ui;
        ui.table_next_column();
        ui.text(stringify!($name));
        ui.table_next_column();
        ui.text($print);
    }};
}

pub struct Metadata {
    cartridge_header: CartridgeHeader,
    cur_subwindow: SubWindow,
}

impl Metadata {
    pub fn new(cartridge_header: CartridgeHeader) -> Self {
        Self {
            cartridge_header,
            cur_subwindow: SubWindow::RomHeader,
        }
    }

    fn rom_header(&mut self, ui: &Ui) {
        let table = ui
            .begin_table_with_flags("rom header", 2, imgui::TableFlags::BORDERS)
            .unwrap();
        tblrow!(ui, title: String::from_utf8_lossy(&self.cartridge_header.game_title));
        table.end()
    }
}

enum SubWindow {
    RomHeader,
}

impl View for Metadata {
    type MutableState = ();

    fn destroy(_: &mut GfxContext) {}

    fn window_title() -> &'static str {
        "Metadata"
    }

    fn construct_window<T: AsRef<str>>(window: imgui::Window<'_, T>) -> imgui::Window<'_, T> {
        window
    }

    fn on_state_changed(&mut self, old_state: Self::MutableState, new_state: &Self::MutableState) {}

    fn menu_bar(&mut self, ui: &Ui, state: &Self::MutableState) {
        if ui.button("header") {
            self.cur_subwindow = SubWindow::RomHeader;
        }
    }

    fn view(&mut self, ui: &Ui, state: &Self::MutableState) {
        match self.cur_subwindow {
            //SubWindow::RomHeader => self.rom_header(ui),
            SubWindow::RomHeader => {}
        }
    }
}
