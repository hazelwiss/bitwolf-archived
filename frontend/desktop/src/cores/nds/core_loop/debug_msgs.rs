use nds_core::rom::CartridgeHeader;

pub enum Data {
    CartridgeHeader(CartridgeHeader),
}

pub enum DebugCtoF {
    Data(Data),
}

pub enum DebugFtoC {
    Running(bool),
    Step(u64),
    StepOver(u64),
    BreakPoint(u64, bool),
}
