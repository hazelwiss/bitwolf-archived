use crate::debug_views::GlobalStateData;

#[derive(Default, Debug, Clone)]
pub struct State {
    pub pc: u32,
    pub gpr: [u32; 16],
}

#[derive(Default, Debug)]
pub struct Registers {
    state: State,
    pc_changed: bool,
}

impl Registers {
    #[inline]
    pub fn pc(&self) -> u32 {
        self.state.pc
    }

    #[inline]
    pub fn gpr(&self) -> &[u32; 16] {
        &self.state.gpr
    }
}

impl GlobalStateData for Registers {
    type State = State;

    #[inline]
    fn on_change(&mut self, old: Self::State) {
        let new = &self.state;
        self.pc_changed = new.pc != old.pc;
    }

    #[inline]
    fn get_state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }
}
