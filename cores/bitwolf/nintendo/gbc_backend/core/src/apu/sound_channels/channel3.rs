use crate::apu::regs::frequency::Frequency;

pub enum OutputLevel {
    Mute,
    Full,
    Half,
    Quarter,
}

pub(in crate::apu) struct Channel3 {
    playback: bool,
    sound_len: u8,
    select_output_level: OutputLevel,
    freq: Frequency,
    wave_pattern: [u8; 16],
}

impl Channel3 {
    pub fn new() -> Self {
        Self {
            playback: false,
            sound_len: 0,
            select_output_level: OutputLevel::Mute,
            freq: Frequency::new(),
            wave_pattern: [0; 16],
        }
    }
}
