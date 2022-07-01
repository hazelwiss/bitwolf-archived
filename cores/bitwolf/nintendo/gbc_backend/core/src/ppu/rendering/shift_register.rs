use super::palette::Colour;

const BUF_LEN: usize = 8;

pub struct ShiftRegister {
    buffer: [Colour; BUF_LEN],
    cur_index: usize,
    len: isize,
}

impl ShiftRegister {
    pub fn new() -> Self {
        Self {
            buffer: [Colour::C0; BUF_LEN],
            cur_index: 0,
            len: 0,
        }
    }

    pub fn pop(&mut self) -> Colour {
        let col = self.buffer[self.cur_index];
        self.buffer[self.cur_index] = Colour::C0;
        self.increment_index();
        if self.len > 0 {
            self.len -= 1;
        }
        col
    }

    pub fn push(&mut self, col: Colour) {
        debug_assert!(
            self.len < BUF_LEN as isize,
            "Cannot push to shift register when it's full!"
        );
        if self.len >= 0 {
            self.buffer[self.cur_index] = col;
            self.increment_index();
        }
        self.len += 1;
    }

    pub fn len(&self) -> isize {
        self.len
    }

    pub fn discard(&mut self, discard: usize) {
        self.len -= discard as isize;
    }

    fn increment_index(&mut self) {
        self.cur_index = (self.cur_index + 1) % BUF_LEN;
    }
}
