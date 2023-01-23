use super::{
    palette::{Index, Palette},
    sprites::SpritePriority,
};

const BUF_LEN: usize = 8;

#[derive(Debug, Clone, Copy)]
pub(super) struct Pixel {
    pub index: Index,
    pub bg_sprite_priority: SpritePriority,
    pub palette: Palette,
}

impl Pixel {
    pub fn empty() -> Self {
        Self {
            index: Index::I0,
            bg_sprite_priority: SpritePriority::SpritePriority,
            palette: Palette::OBP0,
        }
    }
}

#[derive(Debug)]
pub(super) struct ShiftRegister {
    buffer: [Pixel; BUF_LEN],
    cur_index: usize,
    len: usize,
}

impl ShiftRegister {
    pub fn new() -> Self {
        Self {
            buffer: [Pixel::empty(); BUF_LEN],
            cur_index: 0,
            len: 0,
        }
    }

    pub fn pop(&mut self) -> Pixel {
        debug_assert!(self.len > 0, "Cannot pop from an empty shift register.");
        let col = self.buffer[self.cur_index];
        self.buffer[self.cur_index] = Pixel::empty();
        self.increment_index();
        self.len -= 1;
        col
    }

    pub fn push(&mut self, pixel: Pixel) {
        debug_assert!(
            !self.is_full(),
            "Cannot push to shift register when it's full!"
        );
        self.buffer[self.cur_index] = pixel;
        self.increment_index();
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_full(&self) -> bool {
        self.len() >= BUF_LEN
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    fn increment_index(&mut self) {
        self.cur_index = (self.cur_index + 1) % BUF_LEN;
    }
}
