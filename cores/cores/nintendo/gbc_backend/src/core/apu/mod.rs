mod access;
mod control_registers;
mod regs;
mod sound;
mod sound_channels;

pub(crate) use access::APUReg;

use crate::interfaces::AudioInterface;
use sound_channels::{channel1::Channel1, channel2::Channel2, channel3::Channel3};

pub struct APU {
    channel1: Channel1,
    channel2: Channel2,
    channel3: Channel3,
    audio_interface: AudioInterface,
}

impl APU {
    pub fn new(audio_interface: AudioInterface) -> Self {
        Self {
            channel1: Channel1::new(),
            channel2: Channel2::new(),
            channel3: Channel3::new(),
            audio_interface,
        }
    }
}
