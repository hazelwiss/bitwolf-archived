mod audio;
mod config;
mod constraints;
mod debug;
mod input;
mod resources;
mod run;
mod spawn;
mod video;

use anyhow::Result;
use common_frontend::FrontendWrapper;
use std::path::Path;

pub struct GBC {
    com: spawn::frontend::Com,
    resources: resources::Resources,
}

impl GBC {
    pub fn new(path: &Path, wgpu_ctx: &mut imgui::WGPUContext) -> Result<FrontendWrapper> {
        let com = spawn::spawn(path, run::run_normal)?;
        Ok(FrontendWrapper::new(Box::new(Self {
            com,
            resources: resources::Resources::new(wgpu_ctx),
        })))
    }
}

impl common_frontend::Frontend for GBC {}
