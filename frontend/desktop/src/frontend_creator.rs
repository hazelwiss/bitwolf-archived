use crate::backend_types::Types;
use anyhow::Result;
use common_frontend::Frontend;
use std::path::Path;

pub fn spawn(
    backend: Types,
    rom: &Path,
    wgpu_ctx: &mut imgui::WGPUContext,
) -> Result<Box<dyn Frontend>> {
    Ok(match backend {
        Types::NintendoGBC => Box::new(gbc_frontend::GBC::new(rom, wgpu_ctx)?) as Box<dyn Frontend>,
    })
}
