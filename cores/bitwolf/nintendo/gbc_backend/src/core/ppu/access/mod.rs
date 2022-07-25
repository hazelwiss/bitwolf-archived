mod read;
mod write;

pub(crate) enum PPUReg {
    LY,
    LYC,
    SCX,
    SCY,
    WX,
    WY,
    LCDC,
    LCDS,
    BGP,
    OBP0,
    OBP1,
    OAMDMA,
}
