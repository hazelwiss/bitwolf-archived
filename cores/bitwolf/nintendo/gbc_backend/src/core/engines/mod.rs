pub mod interpreter;
pub mod jit;

mod binder;

pub trait Engine {
    type EngineData: Default;
}
