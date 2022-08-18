pub trait Engine {
    type EngineData: Default;
}

pub struct Interpreter;

impl Engine for Interpreter {
    type EngineData = ();
}
