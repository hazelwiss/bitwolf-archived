#![feature(fs_try_exists)]

use std::fs;

const GEN_PATH: &'static str = "gen";

fn main() {
    gen_arm7_table();
    gen_arm9_table();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", GEN_PATH);
}

fn gen_arm9_table() {}

fn gen_arm7_table() {}
