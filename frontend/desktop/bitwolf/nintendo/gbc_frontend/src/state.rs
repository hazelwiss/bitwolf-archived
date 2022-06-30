#[derive(Default)]
pub struct RegisterFile {
    pub pc: u16,
    pub sp: u16,
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

#[derive(Default)]
pub struct State {
    pub reg_file: RegisterFile,
}
