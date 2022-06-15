use crate::backend_types::Types;
use crate::frontend_creator;
use common_frontend::FrontendBox;

pub fn receive(file_dialogue: &mut file_reader::FileReader<Types>, frontend: &mut FrontendBox) {
    if let Some((t, p)) = file_dialogue.retrieve_respons() {
        let val = frontend_creator::spawn(t, &p);
        match val {
            Ok(val) => frontend.swap(val),
            Err(err) => logger::warning!("Unable to open file with error '{err}'"),
        }
    }
}
