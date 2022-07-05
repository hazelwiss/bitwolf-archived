use super::palette::Palette;

#[derive(Debug)]
pub(super) struct SpriteBuffer {
    sprites: [Sprite; 10],
    len: usize,
}

impl SpriteBuffer {
    pub fn new() -> Self {
        Self {
            sprites: [Sprite::none(); 10],
            len: 0,
        }
    }

    pub fn push(&mut self, sprite: Sprite) {
        debug_assert!(
            self.len < self.sprites.len(),
            "Attempted to add a sprite to a full sprite buffer."
        );
        self.sprites[self.len] = sprite;
        self.len += 1;
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn full(&self) -> bool {
        self.len >= self.sprites.len()
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
