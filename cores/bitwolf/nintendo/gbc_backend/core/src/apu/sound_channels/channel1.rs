use crate::apu::regs::{
    frequency::Frequency, pattern_duty::PatternDutyReg, sweep::SweepReg,
    volume_envelope::VolumeEnvelopeReg,
};

pub(in crate::apu) struct Channel1 {
    pub sweep: SweepReg,
    pub pattern_duty: PatternDutyReg,
    pub volume_envelope: VolumeEnvelopeReg,
    pub freq: Frequency,
}

impl Channel1 {
    pub fn new() -> Self {
        Self {
            sweep: SweepReg::new(),
            pattern_duty: PatternDutyReg::new(),
            volume_envelope: VolumeEnvelopeReg::new(),
            freq: Frequency::new(),
        }
    }
}
