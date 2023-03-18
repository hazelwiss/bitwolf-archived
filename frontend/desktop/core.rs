use crate::state::ProgramState;

pub mod nds;

pub trait Core___: Sized {
    fn run_until_sync(state: &mut ProgramState<Self>);

    fn draw_debug(state: &mut ProgramState<Self>, ui: &mut imgui::Ui, io: &imgui::Io);
}

pub struct Core<> {}

pub struct SingleThreaded {}

pub struct MultiThreaded {}
