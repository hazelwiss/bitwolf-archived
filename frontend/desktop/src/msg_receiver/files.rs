use crate::{backend_types::Types, file_reader, frontend_creator};
use common_frontend::FrontendBox;

pub fn receive(
    file_dialogue: &mut file_reader::FileReader<Types>,
    frontend: &mut FrontendBox,
    wgpu_ctx: &mut imgui::WGPUContext,
) {
    if let Some((t, p)) = file_dialogue.retrieve_respons() {
        let val = frontend_creator::spawn(t, &p, wgpu_ctx);
        match val {
            Ok(val) => frontend.swap(val, wgpu_ctx),
            Err(err) => logger::warning!("Unable to open file with error '{err}'"),
        }
    }
}
