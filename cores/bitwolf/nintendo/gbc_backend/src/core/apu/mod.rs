mod access;
mod control_registers;
mod regs;
mod sound;
mod sound_channels;

pub(crate) use access::APUReg;

use sound_channels::{channel1::Channel1, channel2::Channel2, channel3::Channel3};

pub trait AudioBuffer {
    fn queue_sample(&self, sample: i16);
}

pub type Sampler = Box<dyn AudioBuffer>;

pub struct APU {
    sampler: Sampler,
    channel1: Channel1,
    channel2: Channel2,
    channel3: Channel3,
}

impl APU {
    pub fn new(sampler: Sampler) -> Self {
        Self {
            sampler,
            channel1: Channel1::new(),
            channel2: Channel2::new(),
            channel3: Channel3::new(),
        }
    }
}
