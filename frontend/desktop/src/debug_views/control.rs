use super::StaticDV;

#[derive(Default)]
pub struct Control {
    arm9_step: Option<u32>,
    arm7_step: Option<u32>,
}

#[derive(Debug, Default)]
pub struct Emu {
    pub arm9_step: Option<u32>,
    pub arm7_step: Option<u32>,
}

impl StaticDV for Control {
    type Emu = Emu;

    #[inline]
    fn draw(
        &mut self,
        _global_state: &super::GlobalState,
        _: &mut crate::gui::window::Window,
        ui: &imgui::Ui,
        _: &imgui::Io,
    ) {
        if ui.button("step") {
            self.arm9_step = Some(1);
            self.arm7_step = Some(1);
        }
    }

    #[inline]
    fn has_menu_bar(&self) -> bool {
        false
    }

    #[inline]
    fn emu_update(&mut self) -> Option<Self::Emu> {
        if self.arm9_step.is_some() | self.arm7_step.is_some() {
            Some(Emu {
                arm9_step: self.arm9_step.take(),
                arm7_step: self.arm7_step.take(),
            })
        } else {
            None
        }
    }
}
