use crate::GBC;
use common_frontend::update::Update;

impl Update for GBC {
    fn update(&mut self) {
        match &mut self.backend {
            crate::Engine::Interpreter(interp) => {
                for _ in 0..4096 {
                    interp.step()
                }
            }
            crate::Engine::_JIT(_) => todo!(),
        }
    }
}
