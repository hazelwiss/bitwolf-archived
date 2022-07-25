use crate::{debug::messages::CtoF, GBC};
use common_frontend::constraints::update::Update;

impl Update for GBC {
    fn update(&mut self) {
        while let Some(msg) = self.com.msgq.try_recv() {
            match msg {
                CtoF::RegisterFile(reg_file) => self.com.state.reg_file = reg_file,
                CtoF::Control(ctrl) => self.com.state.ctrl = ctrl,
                CtoF::Disassembly(disasm) => self.com.state.disasm = disasm,
            }
        }
    }
}
