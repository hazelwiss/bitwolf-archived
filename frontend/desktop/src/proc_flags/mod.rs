use std::fmt::Display;
use std::path::PathBuf;

use crate::{backend_types, default_backend, frontend_creator};
use argh::FromArgs;
use common_frontend::FrontendBox;
use imgui::Context;

pub enum TypeFromStringErr {
    InvalidString(String),
}

impl Display for TypeFromStringErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            TypeFromStringErr::InvalidString(str) => {
                format!("invalid target identifier string '{str}'")
            }
        })
    }
}

impl std::str::FromStr for backend_types::Types {
    type Err = TypeFromStringErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "nintendo_gbc" => Self::NintendoGBC,
            _ => return Err(TypeFromStringErr::InvalidString(s.to_string())),
        })
    }
}

/// target a specific backend.
#[derive(PartialEq, Debug, FromArgs)]
#[argh(subcommand, name = "target")]
pub struct BackendSubCommand {
    /// set the backend launched with at boot.
    /// Can be the following values: [ nintendo_gbc ]
    #[argh(option)]
    target: backend_types::Types,
    /// set the rom to be loaded.  Remember it should be compatible with the target.
    #[argh(option)]
    rom: PathBuf,
}

/// bitwolf.
#[derive(FromArgs)]
struct ProcFlags {
    /// launch the command with a specific backend.
    #[argh(subcommand)]
    target: Option<BackendSubCommand>,
}

pub struct Environment {
    pub imgui_ctx: Context,
    pub frontend_box: FrontendBox,
}

pub fn env_from_flags() -> Environment {
    let ProcFlags { target } = argh::from_env();

    // Frontend object.
    let frontend = if let Some(target) = target {
        FrontendBox::from_box(frontend_creator::spawn(target.target, &target.rom).expect(""))
    } else {
        FrontendBox::new(default_backend::EmptyFrontend::new())
    };

    // Create imgui rendering window.
    let ctx = imgui::Context::spawn_with_window();

    Environment {
        imgui_ctx: ctx,
        frontend_box: frontend,
    }
}
