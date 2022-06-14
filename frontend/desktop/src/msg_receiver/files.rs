use crate::backend_types::Types;
use common_frontend::{Frontend, FrontendBox};

pub fn receive(file_dialogue: &mut file_reader::FileReader<Types>, frontend: &mut FrontendBox) {
    if let Some((t, p)) = file_dialogue.retrieve_respons() {
        let val: anyhow::Result<Box<dyn Frontend>> = (|| {
            Ok(match t {
                Types::NintendoGBC => Box::new(gbc_frontend::GBC::new(&p)?) as Box<dyn Frontend>,
            })
        })();
        match val {
            Ok(val) => frontend.swap(val),
            Err(err) => logger::warning!("Unable to open file with error '{err}'"),
        }
    }
}
