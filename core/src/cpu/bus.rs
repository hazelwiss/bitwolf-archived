pub trait AccessType {
    const DEBUG: bool;
    const CPU: bool;
}

pub struct CPUAccess;

impl AccessType for CPUAccess {
    const DEBUG: bool = false;
    const CPU: bool = true;
}

pub struct DebugAccess;

impl AccessType for DebugAccess {
    const DEBUG: bool = true;
    const CPU: bool = false;
}
