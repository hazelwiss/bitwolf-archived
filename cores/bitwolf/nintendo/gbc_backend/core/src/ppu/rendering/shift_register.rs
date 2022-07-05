use super::palette::Colour;

const BUF_LEN: usize = 8;

#[derive(Debug)]
pub struct ShiftRegister {
    buffer: [Colour; BUF_LEN],
    cur_index: usize,
    len: usize,
    discard: usize,
}

impl ShiftRegister {
    pub fn new() -> Self {
        Self {
            buffer: [Colour::C0; BUF_LEN],
            cur_index: 0,
            len: 0,
            discard: 0,
        }
    }

    pub fn pop(&mut self) -> Colour {
        let col = self.buffer[self.cur_index];
        self.buffer[self.cur_index] = Colour::C0;
        self.increment_index();
        self.len -= 1;
        col
    }

    pub fn push(&mut self, col: Colour) {
        debug_assert!(
            !self.is_full(),
            "Cannot push to shift register when it's full!"
        );
        if self.discard > 0 {
            self.discard -= 1;
        } else {
            self.buffer[self.cur_index] = col;
            self.increment_index();
            self.len += 1;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_full(&self) -> bool {
        self.len() >= BUF_LEN
    }

    pub fn discard(&mut self, discard: usize) {
        self.discard = discard;
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    fn increment_index(&mut self) {
        self.cur_index = (self.cur_index + 1) % BUF_LEN;
    }
}
