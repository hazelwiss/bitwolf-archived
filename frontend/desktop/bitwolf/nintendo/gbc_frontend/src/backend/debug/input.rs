use std::sync::mpsc::Receiver;

use gbc_backend::{
    engines::interpreter::{self, Interpreter},
    Emu,
};

pub fn recv(emu: &mut Emu<Interpreter>, input: &Receiver<interpreter::input::InputState>) {
    while let Some(input) = input.try_recv().ok() {
        interpreter::input::input(emu, &input)
    }
}
