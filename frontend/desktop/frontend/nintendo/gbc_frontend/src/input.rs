use crate::spawn::backend::Com;

pub struct Input {
    com: *mut Com,
}

unsafe impl Send for Input {}

impl Input {
    pub fn new(com: *mut Com) -> Self {
        Self { com }
    }
}

impl gbc_backend::interfaces::Input for Input {
    fn get_input_state(&self) -> &gbc_backend::input::InputState {
        let com = unsafe { self.com.as_mut().unwrap_unchecked() };
        &com.input_state
    }
}
