use crate::{backend::debug::state::State, state::substates::Control};

pub(super) fn get(state: &State) -> Control {
    Control {
        paused: !state.ctrl.running,
        breakpoints: state.ctrl.break_points.clone(),
    }
}
