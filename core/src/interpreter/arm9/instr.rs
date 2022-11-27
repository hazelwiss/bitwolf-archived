pub mod branch;
pub mod cp;
pub mod data;
pub mod mem;
pub mod misc;

use crate::{Core, Interpreter};
use arm_decode::*;

type CondInstrHandler = fn(&mut Core<Interpreter>, u32);
pub(super) static INSTR_CONDITIONAL: [CondInstrHandler; 1 << 12] =
    include!("../../../gen/arm9_arm_lut");
