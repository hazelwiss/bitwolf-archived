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
