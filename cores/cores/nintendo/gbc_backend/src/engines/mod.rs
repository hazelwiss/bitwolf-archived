pub mod interpreter;

pub trait Engine {
    type EngineData: Default;
}
