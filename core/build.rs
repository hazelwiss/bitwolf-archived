#![feature(fs_try_exists)]

use arm_decode::{CondInstr, FullPrint};
use std::{env, fs};

static CLEAN_VAR: &'static str = "BITWOLF_CLEAN";
static GENERATE_VAR: &'static str = "BITWOLF_GENERATE";

const OUT_DIR: &'static str = "gen";

const OUT_ARM9_ARM: &'static str = "gen/arm9_arm_lut";

fn main() {
    if env::var(CLEAN_VAR).is_ok() {
        macro_rules! remove {
            ($path:ident) => {
                match fs::try_exists($path) {
                    Ok(true) => fs::remove_file($path)
                        .expect(&format!("unable to remove file at path {}", $path)),
                    _ => {}
                }
            };
        }
        remove!(OUT_ARM9_ARM);
    }
    if env::var(GENERATE_VAR).is_ok() {
        generate()
    }
    println!("cargo:rerun-if-env-changed={CLEAN_VAR}");
    println!("cargo:rerun-if-env-changed={GENERATE_VAR}");
    println!("cargo:rerun-if-changed={OUT_DIR}")
}

fn generate_arm9() {
    let processor = arm_decode::Processor {};
    let mut lut: [CondInstr; 1 << 12] = array_init::array_init(|_| CondInstr::Undef);
    let mut output = "[".to_string();
    for i in 0..lut.len() {
        output.push_str(&format!(
            "{},",
            match processor.decode_cond((((i & 0xFF0) << 16) | ((i & 0xF) << 4)) as u32) {
                CondInstr::Msr { r, imm } => format!("data::msr"),
                CondInstr::Mrs { r } => format!("data::mrs"),
                CondInstr::Bx => "branch::bx".to_string(),
                CondInstr::BlxReg => "branch::blx::<false>".to_string(),
                CondInstr::B { link } => format!("branch::b::<{}>", link),
                CondInstr::Clz => "clz".to_string(),
                CondInstr::SatAddSub { sub, doubles } => format!("data::sat_add_sub"),
                CondInstr::DspMul { ty, y } => format!("data::dsp_mul"),
                CondInstr::Bkpt => "misc::bkpt".to_string(),
                CondInstr::Dp {
                    set_flags,
                    opcode,
                    operand,
                } => format!(
                    "data::dp::<{set_flags}, {{ {} }}, {{ {} }}>",
                    opcode.full_print(),
                    operand.full_print(),
                ),
                CondInstr::Mul { acc, set_flags, ty } => format!("data::mul"),
                CondInstr::Swp { byte } => format!("data::swp"),
                CondInstr::Transfer {
                    load,
                    byte,
                    offset_add,
                    operand,
                    addressing,
                } => format!("mem::transfer"),
                CondInstr::MiscTransfer {
                    load,
                    signed,
                    halfword,
                    offset_add,
                    imm,
                    addressing,
                } => format!("mem::misc_transfer"),
                CondInstr::TransferDouble {
                    store,
                    offset_add,
                    imm,
                    addressing,
                } => format!("mem::transfer_double"),
                CondInstr::TransferMult {
                    load,
                    update_base,
                    upwards,
                    privilige_mode,
                    exclude_first,
                } => format!("mem::transfer_multiple"),
                CondInstr::CPTransfer {} => "misc::undef".to_string(),
                CondInstr::CPDp {} => "misc::undef".to_string(),
                CondInstr::CPRegTransfer {} => "misc::undef".to_string(),
                CondInstr::Swi => "misc::swi".to_string(),
                CondInstr::Undef => "misc::undef".to_string(),
                CondInstr::Unpred => "misc::unpred".to_string(),
            }
        ));
    }
    output.push_str("]");
    fs::write(OUT_ARM9_ARM, output).expect("failed to write ARM9 ARM decoding LUT.");
}

fn generate() {
    fs::create_dir_all(OUT_DIR).expect("Unable to create output directory");
    generate_arm9();
}
