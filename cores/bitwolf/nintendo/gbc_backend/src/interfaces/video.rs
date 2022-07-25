pub trait Video {
    fn process_frame(&mut self, frame: &crate::Texture);
}
