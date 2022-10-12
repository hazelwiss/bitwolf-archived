use crate::gui::window::Window;

use super::{DebugView, GlobalState, Ui};
use bitwolf_core::debug::cartridge_info::Header;

#[derive(Default)]
pub struct DVCartridge;

#[derive(Debug, Default)]
pub struct State {
    pub cartridge_header: Header,
}

impl DebugView for DVCartridge {
    type State = State;
    type Conf = ();

    fn draw(
        &mut self,
        state: &mut State,
        _global_state: &GlobalState,
        _window: &mut Window,
        ui: &Ui,
    ) {
        let cart = &state.cartridge_header;
        ui.text(format!("arm9 rom offset: 0x{:08X}", cart.arm9_rom_offset()));
        ui.text(format!(
            "arm9 entry offset: 0x{:08X}",
            cart.arm9_entry_address()
        ));
        ui.text(format!(
            "arm9 ram address: 0x{:08X}",
            cart.arm9_ram_address()
        ));
        ui.text(format!("arm9 size: 0x{:08X}", cart.arm9_size()));
    }

    #[inline]
    fn on_change(&mut self, _old: Self::State, _new: &mut Self::State) {}

    #[inline]
    fn config(&self, _state: &Self::State) -> Option<Self::Conf> {
        None
    }
}
