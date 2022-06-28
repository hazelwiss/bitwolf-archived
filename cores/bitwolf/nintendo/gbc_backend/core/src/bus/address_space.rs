pub struct ROM0(u16);

impl ROM0 {
    pub fn new(adr: u16) -> Self {}
}

pub struct ROM1(u16);

impl ROM1 {
    pub fn new(adr: u16) -> Self {}
}

pub struct VRAM(u16);

impl VRAM {
    pub fn new(adr: u16) -> Self {}
}

pub struct ERAM(u16);

impl ERAM {
    pub fn new(adr: u16) -> Self {}
}

pub struct WRAM0(u16);

impl WRAM0 {
    pub fn new(adr: u16) -> Self {}
}

pub struct WRAM1(u16);

impl WRAM1 {
    pub fn new(adr: u16) -> Self {}
}

pub struct MIRROR(u16);

impl MIRROR {
    pub fn new(adr: u16) -> Self {}
}

pub struct OAM(u16);

impl OAM {
    pub fn new(adr: u16) -> Self {}
}

pub struct Unusable(u16);

impl Unusable {
    pub fn new(adr: u16) -> Self {}
}

pub struct HRAM(u16);

impl HRAM {
    pub fn new(adr: u16) -> Self {}
}
