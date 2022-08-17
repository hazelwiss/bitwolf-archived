pub mod graphics;

pub trait CoreFrontend {
    fn update(frontend: impl Frontend);
}

pub trait Frontend {
    fn draw(graphical: graphics::Graphical);

    fn load_config();
}
