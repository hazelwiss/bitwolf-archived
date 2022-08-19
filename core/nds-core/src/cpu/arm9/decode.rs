//use bitmatch_proc::bitmatch;
//
//use crate::cpu::arm9::registers::RegisterIndex;
//use proc_bitfield::bitfield;
//
//impl From<u8> for RegisterIndex {
//    fn from(v: u8) -> Self {
//        match v & 0xF {
//            0..=12 => Self::GPR(v as u8),
//            13 => Self::SP,
//            14 => Self::LR,
//            15 => Self::PC,
//            16..=u8::MAX => panic!("wrong register index for decoding"),
//        }
//    }
//}
//
//pub mod dp {
//    use super::*;
//
//    #[derive(Clone, Copy)]
//    #[repr(u8)]
//    pub enum Opcode {
//        TEMP = 0,
//    }
//
//    impl From<u8> for Opcode {
//        fn from(v: u8) -> Self {
//            todo!()
//        }
//    }
//
//    bitfield! {
//        #[derive(Clone, Copy)]
//        pub struct ShiftImmediate(pub u8): Debug{}
//    }
//
//    bitfield! {
//        #[derive(Clone, Copy)]
//        pub struct ShiftRegister(pub u8): Debug{}
//    }
//
//    #[derive(Clone, Copy)]
//    pub enum ShiftOperand {
//        Register(ShiftImmediate),
//        Immediate(ShiftRegister),
//    }
//
//    impl From<u8> for ShiftOperand {
//        fn from(_: u8) -> Self {
//            todo!()
//        }
//    }
//
//    bitfield! {
//        #[derive(Clone, Copy, PartialEq, Eq)]
//        pub struct DataProcessing(pub u32) {
//            pub raw: u32 [read_only] @ ..,
//            pub cond: u8 [Cond, read_only] @ 28..=31,
//            pub opcode: u8 [Opcode, read_only] @ 21..=24,
//            pub s: bool [read_only] @ 20,
//            pub rn: u8 [RegisterIndex, read_only] @ 16..=19,
//            pub rm: u8 [RegisterIndex, read_only] @ 0..=3,
//            pub shift: u8 [ShiftOperand, read_only] @ 4..=11,
//        }
//    }
//}
//
//pub mod mul {
//    use super::*;
//
//    bitfield! {
//        pub struct Multiply(pub u32) {
//            pub raw: u32 [read_only] @ ..,
//            pub cond: u8 [Cond, read_only] @ 28..=31,
//            pub accumulate: bool [read_only] @ 21,
//            pub update_flags: bool [read_only] @ 20,
//            pub rd: u8 [RegisterIndex, read_only] @ 16..=19,
//            pub rn: u8 [RegisterIndex, read_only] @ 12..=15,
//            pub rs: u8 [RegisterIndex, read_only] @ 8..=11,
//            pub rm: u8 [RegisterIndex, read_only] @ 0..=3,
//        }
//    }
//}
//
//pub mod mull {
//    use super::*;
//
//    bitfield! {
//        pub struct MultiplyLong(pub u32){
//            pub raw: u32 [read_only] @ ..,
//            pub cond: u8 [Cond, read_only] @ 28..=31,
//            pub unsigned: bool [read_only] @ 22,
//            pub accumulate: bool [read_only] @ 21,
//            pub update_flags: bool [read_only] @ 20,
//            pub rdhi: u8 [RegisterIndex, read_only] @ 16..=19,
//            pub rdlo: u8 [RegisterIndex, read_only] @ 12..=15,
//            pub rs: u8 [RegisterIndex, read_only] @ 8..=11,
//            pub rm: u8 [RegisterIndex, read_only] @ 0..=3,
//        }
//    }
//}
//#[derive(Clone, Copy)]
//#[repr(u8)]
//pub enum Cond {
//    TEST = 0,
//}
//
//impl From<u8> for Cond {
//    fn from(v: u8) -> Self {
//        todo!()
//    }
//}
//
///// Decoded ARM9 opcode.
//pub enum ARM9Instr {
//    /// Data processing
//    Dp(dp::DataProcessing),
//    /// Multiply
//    Mul(mul::Multiply),
//    /// Multiply long
//    Mull(),
//    /// Undefined instruction
//    Undef,
//}
//
//impl ARM9Instr {
//    pub const fn decode(instr: u32) -> Self {
//        bitmatch! {
//            match instr {
//                "????_000?_????_????_????_????_????_????" => Self::Dp(dp::DataProcessing(instr)),
//                _ => Self::Undef,
//            }
//        }
//    }
//}
