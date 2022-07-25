use super::palette::Palette;
use std::collections::BinaryHeap;

const SPRITE_BUFFER_CAPACITY: usize = 10;

#[derive(Debug)]
struct SpriteBufferEntry {
    sprite: Sprite,
    index: usize,
}

impl PartialEq for SpriteBufferEntry {
    fn eq(&self, other: &Self) -> bool {
        let l = other.sprite.x_pos;
        let r = self.sprite.x_pos;
        if l == r {
            other.index.eq(&self.index)
        } else {
            l.eq(&r)
        }
    }
}

impl Eq for SpriteBufferEntry {}

impl PartialOrd for SpriteBufferEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let l = other.sprite.x_pos;
        let r = self.sprite.x_pos;
        if l == r {
            other.index.partial_cmp(&self.index)
        } else {
            l.partial_cmp(&r)
        }
    }
}

impl Ord for SpriteBufferEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let l = other.sprite.x_pos;
        let r = self.sprite.x_pos;
        if l == r {
            other.index.cmp(&self.index)
        } else {
            l.cmp(&r)
        }
    }
}

#[derive(Debug)]
pub(super) struct SpriteBuffer {
    sprites: BinaryHeap<SpriteBufferEntry>,
}

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
        self.sprites.push(SpriteBufferEntry {
            sprite: sprite,
            index: self.sprites.len(),
        });
    }

    pub fn pop(&mut self, x: u8) -> Option<Sprite> {
        let sprite = self.sprites.peek();
        if let Some(sprite) = sprite {
            if sprite.sprite.x_pos <= x {
                self.sprites.pop().map(|s| s.sprite)
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
pub(super) enum SpritePriority {
    SpritePriority,
    BGPriority,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct SpriteFlags {
    pub priority: SpritePriority,
    pub y_flip: bool,
    pub x_flip: bool,
    pub palette: Palette,
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

    pub fn from_u8(val: u8) -> Self {
        let b7 = val & (1 << 7) != 0;
        let b6 = val & (1 << 6) != 0;
        let b5 = val & (1 << 5) != 0;
        let b4 = val & (1 << 4) != 0;
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
