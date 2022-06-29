use gbc_backend::{engines::interpreter, Builder, Core};

pub fn run(builder: Builder) {
    let mut backend = Core::<interpreter::Interpreter>::new(builder);
    loop {
        interpreter::step(&mut backend);
    }
}
