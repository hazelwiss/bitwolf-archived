pub mod regs;

pub trait AudioBuffer {
    fn queue_sample(&self, sample: f32) {}
}

pub type Sampler = Box<dyn AudioBuffer>;

pub struct APU {
    sampler: Sampler,
}

impl APU {
    pub fn new(sampler: Sampler) -> Self {
        Self { sampler }
    }
}
