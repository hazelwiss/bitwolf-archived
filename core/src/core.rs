pub mod arm7;
pub mod arm9;
pub mod engine;

pub struct Core {
    arm9: arm9::ARM9,
    arm7: arm7::ARM7,
}
