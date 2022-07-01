use self::substates::Disassembly;

pub mod substates {
    use std::collections::HashSet;

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
    pub struct Disassembly {
        pub rom: Vec<common_core::disassembly::DisassembledOutput>,
    }

    #[derive(Default)]
    pub struct Control {
        pub paused: bool,
        pub breakpoints: HashSet<u16>,
    }
}

#[derive(Default)]
pub struct State {
    pub reg_file: substates::RegisterFile,
    pub ctrl: substates::Control,
    pub disasm: Disassembly,
}
