use crate::apu::regs::{
    frequency::Frequency, pattern_duty::PatternDutyReg, volume_envelope::VolumeEnvelopeReg,
};

pub(in crate::apu) struct Channel2 {
    pub pattern_duty: PatternDutyReg,
    pub volume_envelope: VolumeEnvelopeReg,
    pub freq: Frequency,
}

impl Channel2 {
    pub fn new() -> Self {
        Self {
            pattern_duty: PatternDutyReg::new(),
            volume_envelope: VolumeEnvelopeReg::new(),
            freq: Frequency::new(),
        }
    }
}
