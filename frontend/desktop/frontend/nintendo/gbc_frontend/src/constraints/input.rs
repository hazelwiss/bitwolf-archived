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
                    let input_state = &mut self.com.input_state;
                    match keycode {
                        VirtualKeyCode::A | VirtualKeyCode::Left => input_state.left = state,
                        VirtualKeyCode::D | VirtualKeyCode::Right => input_state.right = state,
                        VirtualKeyCode::W | VirtualKeyCode::Up => input_state.up = state,
                        VirtualKeyCode::S | VirtualKeyCode::Down => input_state.down = state,
                        VirtualKeyCode::Z => input_state.a = state,
                        VirtualKeyCode::X => input_state.b = state,
                        VirtualKeyCode::P => input_state.start = state,
                        VirtualKeyCode::O => input_state.select = state,
                        _ => {}
                    }
                }
            }
            imgui::Input::MouseButton(_) => {}
        }
        self.com
            .input_sender
            .try_send(self.com.input_state.clone())
            .expect("Unable to send input");
    }
}
