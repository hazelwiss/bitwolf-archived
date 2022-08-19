#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub enum AccessType {
    Debug,
    CPU,
}

pub trait BusAccess {
    const ACCESS_TYPE: AccessType;
}

pub(crate) struct CPUAccess;

impl BusAccess for CPUAccess {
    const ACCESS_TYPE: AccessType = AccessType::CPU;
}

pub struct DebugAccess;

impl BusAccess for DebugAccess {
    const ACCESS_TYPE: AccessType = AccessType::Debug;
}
