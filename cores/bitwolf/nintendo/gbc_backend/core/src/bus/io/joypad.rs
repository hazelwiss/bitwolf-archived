use crate::bus::Bus;

pub struct Joypad {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub start: bool,
    pub select: bool,
    pub b: bool,
    pub a: bool,
    pub select_direction: bool,
    pub select_action: bool,
}

impl Joypad {
    pub(super) fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            start: false,
            select: false,
            b: false,
            a: false,
            select_direction: false,
            select_action: false,
        }
    }

    pub(super) fn as_byte(&self) -> u8 {
        let mut byte = 0;
        byte |= (self.select_action as u8) << 5;
        byte |= (self.select_direction as u8) << 4;
        if self.select_action {
            byte |= (!self.down as u8) << 3;
            byte |= (!self.up as u8) << 2;
            byte |= (!self.left as u8) << 1;
            byte |= !self.right as u8;
        } else if self.select_direction {
            byte |= (!self.start as u8) << 3;
            byte |= (!self.select as u8) << 2;
            byte |= (!self.b as u8) << 1;
            byte |= !self.a as u8;
        }
        byte
    }

    pub(super) fn set_action(&mut self, val: bool) {
        self.select_action = val;
    }

    pub(super) fn set_direction(&mut self, val: bool) {
        self.select_direction = val;
    }
}

impl Bus {
    pub(super) fn write_joypad(&mut self, val: u8) {
        let action = val & (1 << 5) != 0;
        let direction = val & (1 << 4) != 0;
        self.io.joypad.set_action(action);
        self.io.joypad.set_direction(direction);
    }

    pub(super) fn read_joypad(&mut self) -> u8 {
        self.io.joypad.as_byte()
    }
}
