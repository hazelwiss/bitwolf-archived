use crate::spawn::backend::Com;

pub struct Video {
    com: *mut Com,
}

unsafe impl Send for Video {}

impl Video {
    pub fn new(com: *mut Com) -> Self {
        Self { com }
    }
}

impl gbc_backend::interfaces::Video for Video {
    fn process_frame(&mut self, frame: &gbc_backend::Texture) {
        let com = unsafe { self.com.as_mut().unwrap_unchecked() };
        *com.fb_writer.get().write() = frame.clone();
        while let Ok(input_state) = com.input_recv.try_recv() {
            com.input_state = input_state;
        }
    }
}
