use crate::{
    debug_views::{GlobalState, StaticDV, Ui},
    gui::window::Window,
};
use imgui::Io;

#[derive(Default)]
pub struct Cartridge;

impl StaticDV for Cartridge {
    type Emu = ();

    #[inline]
    fn draw(&mut self, global_state: &GlobalState, _window: &mut Window, ui: &Ui, _io: &Io) {
        let header = &global_state.cartridge.get_header();
        ui.text(format!("arm9 rom offset: 0x{:08X}", header.arm9_rom_adr()));
        ui.text(format!("arm9 entry: 0x{:08X}", header.arm9_entry()));
        ui.text(format!(
            "arm9 load address: 0x{:08X}",
            header.arm9_load_adr()
        ));
        ui.text(format!("arm9 size: 0x{:08X}", header.arm9_size()));
    }

    #[inline]
    fn has_menu_bar(&self) -> bool {
        false
    }

    #[inline]
    fn emu_update(&mut self) -> Option<Self::Emu> {
        None
    }
}
