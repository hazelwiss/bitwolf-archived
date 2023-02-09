pub enum RegisterInfo {
    NDS { arm9_gpr: [u32; 16] },
}

pub trait Core {
    fn run_for_cycles(&mut self, cycles: u64);

    fn register_info(&self) -> RegisterInfo;

    fn step(&mut self);
}
