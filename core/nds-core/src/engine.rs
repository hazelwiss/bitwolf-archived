pub trait Engine {
    type EngineData;
}

pub struct Interpreter;

impl Engine for Interpreter {
    type EngineData = ();
}
