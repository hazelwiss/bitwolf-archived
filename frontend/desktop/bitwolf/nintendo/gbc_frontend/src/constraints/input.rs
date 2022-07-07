use crate::GBC;
use common_frontend::constraints::input::Input;
use imgui::win_api::event::{ElementState, VirtualKeyCode};

impl Input for GBC {
    fn input(&mut self, input: imgui::Input) {
        match input {
            imgui::Input::Keyboard(input) => {
                let state = match input.state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                };
                if let Some(keycode) = input.virtual_keycode {
                    match keycode {
                        VirtualKeyCode::A | VirtualKeyCode::Left => self.input_state.left = state,
                        VirtualKeyCode::D | VirtualKeyCode::Right => self.input_state.right = state,
                        VirtualKeyCode::W | VirtualKeyCode::Up => self.input_state.up = state,
                        VirtualKeyCode::S | VirtualKeyCode::Down => self.input_state.down = state,
                        VirtualKeyCode::Z => self.input_state.a = state,
                        VirtualKeyCode::X => self.input_state.b = state,
                        VirtualKeyCode::P => self.input_state.start = state,
                        VirtualKeyCode::O => self.input_state.select = state,
                        _ => {}
                    }
                }
            }
            imgui::Input::MouseButton(_) => {}
        }
        self.input
            .try_send(self.input_state.clone())
            .expect("Unable to send input");
    }
}
