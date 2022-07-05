use super::palette::Palette;
use std::collections::BinaryHeap;

const SPRITE_BUFFER_CAPACITY: usize = 10;

impl SpriteBuffer {
    pub fn new() -> Self {
        Self {
            sprites: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, sprite: Sprite) {
        debug_assert!(
            self.len() < SPRITE_BUFFER_CAPACITY,
            "Attempted to add a sprite to a full sprite buffer."
        );
        self.sprites.push(sprite);
    }

    pub fn pop(&mut self, x: u8) -> Option<Sprite> {
        let sprite = self.sprites.peek();
        if let Some(sprite) = sprite {
            if sprite.x_pos <= x {
                Some(unsafe { self.sprites.pop().unwrap_unchecked() })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        *self = SpriteBuffer::new();
    }

    pub fn full(&self) -> bool {
        self.len() >= SPRITE_BUFFER_CAPACITY
    }

    pub fn len(&self) -> usize {
        self.sprites.len()
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub(super) enum SpritePriority {
    SpritePriority = 0,
    BGPriority = 1,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct SpriteFlags {
    priority: SpritePriority,
    y_flip: bool,
    x_flip: bool,
    palette: Palette,
}

impl SpriteFlags {
    fn new() -> Self {
        Self {
            priority: SpritePriority::SpritePriority,
            y_flip: false,
            x_flip: false,
            palette: Palette::OBP0,
        }
    }

    pub fn from_byte(byte: u8) -> Self {
        let b7 = byte & (1 << 7) != 0;
        let b6 = byte & (1 << 6) != 0;
        let b5 = byte & (1 << 5) != 0;
        let b4 = byte & (1 << 4) != 0;
        let priority = if b7 {
            SpritePriority::BGPriority
        } else {
            SpritePriority::SpritePriority
        };
        let y_flip = b6;
        let x_flip = b5;
        let palette = if b4 { Palette::OBP1 } else { Palette::OBP0 };
        Self {
            priority,
            y_flip,
            x_flip,
            palette,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Sprite {
    pub y_pos: u8,
    pub x_pos: u8,
    pub tile_num: u8,
    pub flags: SpriteFlags,
}

impl Sprite {
    pub fn none() -> Self {
        Self {
            y_pos: 0,
            x_pos: 0,
            tile_num: 0,
            flags: SpriteFlags::new(),
        }
    }
}

impl PartialEq for Sprite {
    fn eq(&self, other: &Self) -> bool {
        other.x_pos.eq(&self.x_pos)
    }
}

impl Eq for Sprite {}

impl PartialOrd for Sprite {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.x_pos.partial_cmp(&self.x_pos)
    }
}

impl Ord for Sprite {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.x_pos.cmp(&self.x_pos)
    }
}

#[derive(Debug)]
pub(super) struct SpriteBuffer {
    sprites: BinaryHeap<Sprite>,
}
