use crate::frontend::nds::{DebugGuiSetup, Interp, Jit};
use nds::{Core, Engine, Interpreter};

pub struct Metadata {
    engine_print: String,
    rom_name: String,
    rom_size: u64,
}

impl Metadata {
    fn setup<T: Engine, const JIT: bool>(core: &Core<T>) -> Self {
        Self {
            engine_print: if JIT { "jit" } else { "interpreter" }.to_string(),
            rom_name: "".to_string(),
            rom_size: 0,
        }
    }
}

impl DebugGuiSetup for Metadata {
    fn setup_interp(core: &Interp) -> Self {
        Self::setup::<_, false>(core)
    }

    fn setup_jit(core: &Jit) -> Self {
        unimplemented!()
    }

    fn draw(&mut self, ui: &imgui::Ui, io: &imgui::Io) {
        {
            let _table = ui.begin_table("__metadata", 2);
            ui.table_next_column();
            ui.text("engine");
            ui.table_next_column();
            ui.text(&self.engine_print);
            ui.table_next_column();
            ui.text("rom name");
            ui.table_next_column();
            ui.text(&self.rom_name);
            ui.table_next_column();
            ui.text("rom size");
            ui.table_next_column();
            ui.text(self.rom_size.to_string());
        }
    }
}
