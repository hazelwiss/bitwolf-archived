#![feature(fs_try_exists)]

use arm_decode::CondInstr;
use std::fs;

const OUT_DIR: &str = "gen";
const OUT_ARM9_ARM: &str = "gen/arm9_arm_lut";

fn main() {
    match fs::try_exists(OUT_ARM9_ARM) {
        Ok(false) => generate(),
        _ => {}
    }
    println!("cargo:rerun-if-changed={OUT_DIR}")
}

fn generate_arm9() {
    let processor = arm_decode::Processor {};
    let mut output = "[".to_string();
    for i in 0..(1 << 12) {
        output.push_str(&format!(
            "{},",
            match processor.decode_cond((((i & 0xFF0) << 16) | ((i & 0xF) << 4)) as u32) {
                CondInstr::Msr(arg) => format!("data::msr::<{{ {arg} }}>"),
                CondInstr::Mrs(arg) => format!("data::mrs::<{{ {arg} }}>"),
                CondInstr::Bx => "branch::bx".to_string(),
                CondInstr::BlxReg => "branch::blx::<false>".to_string(),
                CondInstr::B(arg) => format!("branch::b::<{{ {arg} }}>"),
                CondInstr::Clz => "data::clz".to_string(),
                CondInstr::QArith(arg) => format!("coproc::qarith::<{{ {arg} }}>"),
                CondInstr::DspMul(arg) => format!("coproc::dsp_mul::<{{ {arg} }}>"),
                CondInstr::Bkpt => "misc::bkpt".to_string(),
                CondInstr::Dp(arg) => format!("data::dp::<{{ {arg} }}>"),
                CondInstr::Mul(arg) => format!("data::mul::<{{ {arg} }}>"),
                CondInstr::Swp(arg) => format!("data::swp::<{{ {arg} }}>"),
                CondInstr::Transfer(arg) => format!("mem::transfer::<{{ {arg} }}>"),
                CondInstr::MiscTransfer(arg) => format!("mem::misc_transfer::<{{ {arg} }}>"),
                CondInstr::TransferDouble(arg) => format!("mem::transfer_double::<{{ {arg} }}>"),
                CondInstr::TransferMult(arg) => format!("mem::transfer_multiple::<{{ {arg} }}>"),
                CondInstr::CPTransfer => "misc::undef".to_string(),
                CondInstr::CPDp => "misc::undef".to_string(),
                CondInstr::CPRegTransfer => "misc::undef".to_string(),
                CondInstr::Swi => "misc::swi".to_string(),
                CondInstr::Undef => "misc::undef".to_string(),
                CondInstr::Unpred => "misc::unpred".to_string(),
            }
        ));
    }
    output.push(']');
    fs::write(OUT_ARM9_ARM, output).expect("failed to write ARM9 ARM decoding LUT.");
}

fn generate() {
    fs::create_dir_all(OUT_DIR).expect("Unable to create output directory");
    generate_arm9();
}
