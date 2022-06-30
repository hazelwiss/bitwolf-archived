use super::{super::step, Emu, Interpreter};

pub fn into(emu: &mut Emu<Interpreter>) {
    step(emu);
}

pub fn over(emu: &mut Emu<Interpreter>) {
    step(emu);
}
