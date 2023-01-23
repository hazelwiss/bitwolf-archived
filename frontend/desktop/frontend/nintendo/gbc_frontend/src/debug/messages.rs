use super::state::substates::{Control, Disassembly, RegisterFile};

pub enum FtoC {
    SetPausedState(bool),
    Step(u64),
    StepOver,
}

pub enum CtoF {
    RegisterFile(RegisterFile),
    Control(Control),
    Disassembly(Disassembly),
}
