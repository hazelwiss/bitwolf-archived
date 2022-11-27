use crate::debug_views::GlobalStateData;
use bitwolf_core::debug::cartridge_info::Header;

#[derive(Debug, Default, Clone)]
pub struct State {
    pub cartridge_header: Header,
}

#[derive(Default, Debug)]
pub struct Cartridge {
    state: State,
}

impl Cartridge {
    pub fn get_header(&self) -> &Header {
        &self.state.cartridge_header
    }
}

impl GlobalStateData for Cartridge {
    type State = State;

    #[inline]
    fn on_change(&mut self, _: Self::State) {}

    #[inline]
    fn get_state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }
}
