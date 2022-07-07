#[repr(u8)]
#[derive(Clone, Copy)]
pub(crate) enum TileMapArea {
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
pub(crate) enum TileDataArea {
    A8800_97FF = 0,
    A8000_8FFF = 1,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub(crate) enum OBJSize {
    S8x8 = 0,
    S8x16 = 1,
}

pub(crate) struct LCDC {
    pub enable: bool,
    pub window_tile_map_area: TileMapArea,
    pub window_enable: bool,
    pub bg_and_window_tile_data_area: TileDataArea,
    pub bg_tile_map_area: TileMapArea,
    pub obj_size: OBJSize,
    pub obj_enable: bool,
    pub bg_and_window_enable: bool,
}
