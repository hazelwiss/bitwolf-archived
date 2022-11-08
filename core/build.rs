#![feature(fs_try_exists)]

use arm_decode::CondInstr;
use std::fs;

const OUT_DIR: &str = "gen";
const OUT_ARM9_ARM: &str = "gen/arm9_arm_lut";

fn main() {
    #[allow(clippy::single_match)]
    match fs::try_exists(OUT_ARM9_ARM) {
        Ok(false) => generate(),
        _ => {}
    }
    println!("cargo:rerun-if-changed={OUT_DIR}")
}

fn generate_arm9() {
    let processor = arm_decode::Processor {};
    let mut output = "[".to_string();
    let cp_mod = "cp";
    let mem_mod = "mem";
    let data_mod = "data";
    let misc_mod = "misc";
    let branch_mod = "branch";

    for i in 0..(1 << 12) {
        output.push_str(&format!(
            "{},",
            match processor.decode_cond((((i & 0xFF0) << 16) | ((i & 0xF) << 4)) as u32) {
                CondInstr::Msr(arg) => format!("{data_mod}::msr::<{{ {arg} }}>"),
                CondInstr::Mrs(arg) => format!("{data_mod}::mrs::<{{ {arg} }}>"),
                CondInstr::Bx => format!("{branch_mod}::bx"),
                CondInstr::BlxReg => format!("{branch_mod}::blx::<false>"),
                CondInstr::B(arg) => format!("{branch_mod}::b::<{{ {arg} }}>"),
                CondInstr::Clz => format!("{data_mod}::clz"),
                CondInstr::QArith(arg) => format!("{cp_mod}::qarith::<{{ {arg} }}>"),
                CondInstr::DspMul(arg) => format!("{cp_mod}::dsp_mul::<{{ {arg} }}>"),
                CondInstr::Bkpt => format!("{misc_mod}::bkpt"),
                CondInstr::Dp(arg) => format!("{data_mod}::dp::<{{ {arg} }}>"),
                CondInstr::Mul(arg) => format!("{data_mod}::mul::<{{ {arg} }}>"),
                CondInstr::Swp(arg) => format!("{data_mod}::swp::<{{ {arg} }}>"),
                CondInstr::Transfer(arg) => format!("{mem_mod}::transfer::<{{ {arg} }}>"),
                CondInstr::MiscTransfer(arg) => format!("{mem_mod}::misc_transfer::<{{ {arg} }}>"),
                CondInstr::TransferDouble(arg) =>
                    format!("{mem_mod}::transfer_double::<{{ {arg} }}>"),
                CondInstr::TransferMult(arg) =>
                    format!("{mem_mod}::transfer_multiple::<{{ {arg} }}>"),
                CondInstr::CPTransfer(arg) => format!("{cp_mod}::cp_transfer::<{{ {arg} }}>"),
                CondInstr::CPDp => format!("{cp_mod}::cdp"),
                CondInstr::CPMov(arg) => format!("{cp_mod}::cp_mov::<{{ {arg} }}>"),
                CondInstr::Swi => format!("{misc_mod}::swi"),
                CondInstr::Undef => format!("{misc_mod}::undef"),
                CondInstr::Unpred => format!("{misc_mod}::unpred"),
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
