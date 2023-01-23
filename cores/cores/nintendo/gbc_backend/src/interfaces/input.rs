use crate::input::InputState;

pub trait Input {
    fn get_input_state(&self) -> &InputState;
}
