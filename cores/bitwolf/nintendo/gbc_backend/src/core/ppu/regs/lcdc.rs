#[repr(u8)]
#[derive(Clone, Copy)]
pub(in crate::core::ppu) enum TileMapArea {
    A9800_9BFF = 0,
    A9C00_9FFF = 1,
}

impl TileMapArea {
    pub fn get_map_base_adr(&self) -> u16 {
        match self {
            TileMapArea::A9800_9BFF => 0x9800,
            TileMapArea::A9C00_9FFF => 0x9C00,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub(in crate::core::ppu) enum TileDataArea {
    A8800_97FF = 0,
    A8000_8FFF = 1,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub(in crate::core::ppu) enum OBJSize {
    S8x8 = 0,
    S8x16 = 1,
}

pub(in crate::core::ppu) struct LCDC {
    pub enable: bool,
    pub window_tile_map_area: TileMapArea,
    pub window_enable: bool,
    pub bg_and_window_tile_data_area: TileDataArea,
    pub bg_tile_map_area: TileMapArea,
    pub obj_size: OBJSize,
    pub obj_enable: bool,
    pub bg_and_window_enable: bool,
}

impl LCDC {
    pub fn new() -> Self {
        LCDC {
            enable: false,
            window_tile_map_area: TileMapArea::A9800_9BFF,
            window_enable: false,
            bg_and_window_tile_data_area: TileDataArea::A8800_97FF,
            bg_tile_map_area: TileMapArea::A9800_9BFF,
            obj_size: OBJSize::S8x8,
            obj_enable: false,
            bg_and_window_enable: false,
        }
    }

    pub fn from_u8(val: u8, ppu: &mut crate::core::ppu::PPU) -> Self {
        let enable = val & (1 << 7) != 0;
        if !enable {
            ppu.reset();
        }
        let window_tile_map_area = if val & (1 << 6) == 0 {
            TileMapArea::A9800_9BFF
        } else {
            TileMapArea::A9C00_9FFF
        };
        let window_enable = val & (1 << 5) != 0;
        let bg_and_window_tile_data_area = if val & (1 << 4) == 0 {
            TileDataArea::A8800_97FF
        } else {
            TileDataArea::A8000_8FFF
        };
        let bg_tile_map_area = if val & (1 << 3) == 0 {
            TileMapArea::A9800_9BFF
        } else {
            TileMapArea::A9C00_9FFF
        };
        let obj_size = if val & (1 << 2) == 0 {
            OBJSize::S8x8
        } else {
            OBJSize::S8x16
        };
        let obj_enable = val & (1 << 1) != 0;
        let bg_and_window_enable = val & 1 != 0;
        LCDC {
            enable,
            window_tile_map_area,
            window_enable,
            bg_and_window_tile_data_area,
            bg_tile_map_area,
            obj_size,
            obj_enable,
            bg_and_window_enable,
        }
    }

    pub fn as_u8(&self) -> u8 {
        let mut res = 0;
        res |= (self.enable as u8) << 7;
        res |= (self.window_tile_map_area as u8) << 6;
        res |= (self.window_enable as u8) << 5;
        res |= (self.bg_and_window_tile_data_area as u8) << 4;
        res |= (self.bg_tile_map_area as u8) << 3;
        res |= (self.obj_size as u8) << 2;
        res |= (self.obj_enable as u8) << 1;
        res |= self.bg_and_window_enable as u8;
        res
    }
}
