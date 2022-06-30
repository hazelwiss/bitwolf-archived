use crate::GBC;
use common_frontend::constraints::update::Update;

impl Update for GBC {
    fn update(&mut self) {
        while let Some(msg) = self.bdq.try_recv() {}
    }
}
