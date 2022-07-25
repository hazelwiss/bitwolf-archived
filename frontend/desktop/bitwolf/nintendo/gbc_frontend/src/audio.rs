use libaudio::AudioContext;

pub struct Audio {
    ctx: AudioContext<i16, 32>,
}

impl Audio {
    pub fn new() -> Self {
        Self {
            ctx: libaudio::AudioBuilder::new().play(),
        }
    }
}

impl gbc_backend::interfaces::Audio for Audio {
    fn handle_sample(&mut self, sample: i16) {
        self.ctx.push_sample(sample)
    }
}
