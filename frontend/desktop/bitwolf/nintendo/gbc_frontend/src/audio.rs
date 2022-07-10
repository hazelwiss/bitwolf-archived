use gbc_backend::AudioBuffer;
use libaudio::AudioContext;

pub struct AudioSampler {
    ctx: AudioContext<f32, 32>,
}

impl AudioSampler {
    pub fn new() -> Self {
        Self {
            ctx: libaudio::AudioBuilder::new().play(),
        }
    }
}

impl AudioBuffer for AudioSampler {
    fn queue_sample(&self, sample: f32) {
        self.ctx.push_sample(sample)
    }
}
