use gbc_backend::{Builder, Core, Interpreter};

pub fn run(builder: Builder) {
    let mut backend = Core::<Interpreter>::new(builder);
    loop {
        backend.step()
    }
}
