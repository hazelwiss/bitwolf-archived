pub trait Audio {
    fn handle_sample(&mut self, sample: i16);
}
