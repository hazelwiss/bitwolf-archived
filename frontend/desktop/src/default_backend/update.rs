use super::EmptyFrontend;
use common_frontend::update::Update;

impl Update for EmptyFrontend {
    fn update(&mut self) {}
}
