use super::NDSFrontend;

pub struct DebugState {
    pub disassemble_arm9: (),
}

impl Default for DebugState {
    fn default() -> Self {
        Self {
            disassemble_arm9: (),
        }
    }
}

pub fn sync_debug_state(nds: &mut NDSFrontend) {
    //let read32 = nds_core::bus::read32(&mut nds.nds_core, 0);
    
}
