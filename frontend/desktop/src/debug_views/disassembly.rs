use imgui::{Io, TableBgTarget, TableFlags, TableRowFlags};

use crate::gui::window::Window;

use super::{DebugView, GlobalState, Ui};

macro_rules! memory_sections {
    ($($name:ident => $start:literal, $end:expr);* $(;)?) => {
        enum MemorySection {
            $($name,)*
        }

        impl MemorySection {
            #[inline]
            fn start_and_len(&self) -> (u32, usize) {
                (self.start(), self.len())
            }

            #[inline]
            fn start(&self) -> u32 {
                match self {
                    $(
                        Self::$name => $start,
                    )*
                }
            }

            #[inline]
            fn len(&self) -> usize {
                match self {
                    $(
                        Self::$name => $end - $start,
                    )*
                }
            }

            #[inline]
            fn from_adr(&self, adr: u32) -> Option<(Self, u32)> {
                match adr {
                    $(
                        $start..=$end => Some((Self::$name, adr & (self.start() - 1))),
                    )*
                    _ => None
                }
            }
        }
    };
}

memory_sections! {
    MainMemory => 0x0200_0000, 0x0240_0000;
}

impl Default for MemorySection {
    fn default() -> Self {
        Self::MainMemory
    }
}

enum ViewType {
    Arm9,
    Arm7,
}

impl Default for ViewType {
    fn default() -> Self {
        Self::Arm9
    }
}

#[derive(Default)]
pub struct DVDisasm {
    mem_section: MemorySection,
    view_type: ViewType,
    line_cnt: usize,
    hovered: bool,
    rel_adr: u32,
    abs_adr: u32,
}

impl DVDisasm {
    fn scroll(&mut self, scroll: f32) {
        self.rel_adr = if scroll.is_sign_positive() {
            self.rel_adr.saturating_sub(scroll as u32 * 4)
        } else {
            self.rel_adr.saturating_add(-scroll as u32 * 4)
        };
    }
}

#[derive(Default, Debug)]
pub struct State {
    pub disasm: Vec<(String, Vec<u8>)>,
}

#[derive(Debug, Default)]
pub struct Conf {
    pub start_adr: u32,
    pub line_cnt: usize,
}

impl DebugView for DVDisasm {
    type State = State;
    type Conf = Conf;

    #[inline]
    fn draw(
        &mut self,
        state: &mut State,
        global_state: &GlobalState,
        _window: &mut Window,
        ui: &Ui,
        io: &Io,
    ) {
        if self.hovered {
            let wheel = io.mouse_wheel;
            self.scroll(wheel);
        }
        ui.menu_bar(|| {
            if ui.button("ARM9") {
                self.view_type = ViewType::Arm9
            }
            if ui.button("ARM7") {
                self.view_type = ViewType::Arm7
            }
        });
        if ui.button("Go to PC") {
            if let Some((section, rel)) = self.mem_section.from_adr(global_state.registers.pc()) {
                self.mem_section = section;
                self.rel_adr = rel;
            }
        }
        let (start, end) = self.mem_section.start_and_len();
        imgui::ChildWindow::new("disasm")
            .border(true)
            .scroll_bar(false)
            .scrollable(false)
            .size([-20.0, 0.0])
            .build(ui, || {
                let column_height = ui.current_font_size() + 2.0;
                let [_, window_height] = ui.content_region_max();
                let tmp = ("".to_string(), vec![]);
                self.line_cnt = ((window_height / column_height) as usize)
                    .clamp(1, end.saturating_sub(self.rel_adr as usize));
                self.abs_adr = start + self.rel_adr;
                let _table = ui
                    .begin_table_with_flags("disasm_tbl", 3, TableFlags::SIZING_FIXED_FIT)
                    .unwrap();
                for i in 0..self.line_cnt {
                    let (str, bytes) = if let Some(val) = state.disasm.get(i) {
                        val
                    } else {
                        &tmp
                    };
                    ui.table_next_row_with_height(TableRowFlags::empty(), column_height);
                    ui.table_next_column();
                    let adr = self.abs_adr + i as u32 * 4;
                    if adr == global_state.registers.pc() {
                        ui.table_set_bg_color(TableBgTarget::ROW_BG0, [0.45, 0.45, 0.45, 0.5]);
                    }
                    ui.text(format!("{adr:08X}:"));
                    ui.table_next_column();
                    ui.text_colored([0.2, 0.2, 0.2, 1.0], format!("{bytes:02X?}"));
                    ui.table_next_column();
                    ui.text_colored([0.0, 0.6, 0.0, 1.0], str);
                }
            });
        self.hovered = ui.is_item_hovered();
        ui.same_line();
        imgui::ChildWindow::new("scroll")
            .scrollable(true)
            .scroll_bar(true)
            .always_vertical_scrollbar(true)
            .build(ui, || {});
    }

    #[inline]
    fn has_menu_bar(&self) -> bool {
        true
    }

    #[inline]
    fn on_change(&mut self, _old: Self::State, _new: &mut Self::State) {}

    #[inline]
    fn config(&self) -> Option<Self::Conf> {
        Some(Conf {
            start_adr: self.abs_adr,
            line_cnt: self.line_cnt,
        })
    }
}
