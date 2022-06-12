mod windows;

use gbc::{
    engines::{interpreter::Interpreter, jit::JIT},
    Core,
};

pub enum Engine {
    _JIT(Core<JIT>),
    Interp(Core<Interpreter>),
}

pub struct Context {
    engine: Engine,
    _windows: windows::Windows,
}

impl Context {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine,
            _windows: windows::Windows::new(),
        }
    }

    pub fn update(&mut self) {
        match &mut self.engine {
            Engine::_JIT(_) => todo!(),
            Engine::Interp(interp) => {
                interp.step();
            }
        }
    }
}
