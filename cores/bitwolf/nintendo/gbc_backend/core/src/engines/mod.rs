pub mod interpreter;
pub mod jit;

mod debug;

pub trait Engine {
    type EngineData: Default;
}
