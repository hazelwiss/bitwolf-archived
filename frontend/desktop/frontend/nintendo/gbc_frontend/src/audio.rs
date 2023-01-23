use libaudio::AudioContext;

pub struct Audio {
    ctx: AudioContext<f32, 32>,
}

impl Audio {
    pub fn new() -> Self {
        Self {
            ctx: libaudio::AudioBuilder::new().play(),
        }
    }
}

impl gbc_backend::interfaces::Audio for Audio {
    fn handle_sample(&mut self, sample: f32) {
        self.ctx.push_sample(sample)
    }
}
