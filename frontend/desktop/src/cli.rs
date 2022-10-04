use std::path::PathBuf;

use clap::Parser;

/// Bitwolf.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// ROM to load initially.
    #[arg(short, long)]
    pub rom: Option<PathBuf>,

    /// Config options path. (defaults to system path).
    #[arg(long)]
    pub config: Option<PathBuf>,
}

pub fn parse() -> CliArgs {
    CliArgs::parse()
}
