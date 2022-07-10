use crate::backend_types::Types;
use anyhow::Result;
use common_frontend::FrontendWrapper;
use std::path::Path;

pub fn spawn(
    backend: Types,
    rom: &Path,
    wgpu_ctx: &mut imgui::WGPUContext,
) -> Result<FrontendWrapper> {
    Ok(match backend {
        Types::NintendoGBC => gbc_frontend::GBC::new(rom, wgpu_ctx)?,
    })
}
