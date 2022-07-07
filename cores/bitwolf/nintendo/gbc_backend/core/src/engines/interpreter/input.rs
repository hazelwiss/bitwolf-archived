use super::{Emu, Interpreter};

#[derive(Clone)]
pub struct InputState {
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
    pub down: bool,
    pub up: bool,
    pub left: bool,
    pub right: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            a: false,
            b: false,
            start: false,
            select: false,
            down: false,
            up: false,
            left: false,
            right: false,
        }
    }
}

pub fn input(emu: &mut Emu<Interpreter>, state: &InputState) {
    let joypad = emu.bus.get_joypad_mut();
    joypad.a = state.a;
    joypad.b = state.b;
    joypad.left = state.left;
    joypad.right = state.right;
    joypad.up = state.up;
    joypad.down = state.down;
    joypad.select = state.select;
    joypad.start = state.start;
}
