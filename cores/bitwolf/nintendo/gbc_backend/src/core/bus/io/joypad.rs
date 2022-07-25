use crate::{core::bus::Bus, interfaces::InputInterface};

pub struct Joypad {
    input_interface: InputInterface,
    select_action: bool,
    select_direction: bool,
}

impl Joypad {
    pub(super) fn new(input_interface: InputInterface) -> Self {
        Self {
            input_interface,
            select_action: false,
            select_direction: false,
        }
    }

    pub(super) fn as_u8(&self) -> u8 {
        let mut byte = 0;
        let input = self.input_interface.get_input_state();
        byte |= (self.select_action as u8) << 5;
        byte |= (self.select_direction as u8) << 4;
        if self.select_action {
            byte |= (!input.down as u8) << 3;
            byte |= (!input.up as u8) << 2;
            byte |= (!input.left as u8) << 1;
            byte |= !input.right as u8;
        } else if self.select_direction {
            byte |= (!input.start as u8) << 3;
            byte |= (!input.select as u8) << 2;
            byte |= (!input.b as u8) << 1;
            byte |= !input.a as u8;
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
        self.io.joypad.as_u8()
    }
}
