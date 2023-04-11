use crate::frontend::nds::{DebugGlobalState, DebugGui};
use imgui::{Io, TableBgTarget, TableFlags, TableRowFlags};

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
            fn from_adr(adr: u32) -> Option<(Self, u32)> {
                match adr {
                    $(
                        $start..=$end => Some((Self::$name, adr & ($start - 1))),
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

#[derive(Default)]
pub struct Disasm {
    mem_section: MemorySection,
    line_cnt: usize,
    hovered: bool,
    rel_adr: u32,
    abs_adr: u32,
}

pub struct Instr {
    pub bytes: Box<[u8]>,
    pub print: String,
}

#[derive(Default)]
pub struct State {
    pub line: u32,
    pub instr: Box<[Instr]>,
}

#[derive(Default, Clone)]
pub struct Conf {
    pub line: u32,
    pub count: usize,
}

impl Disasm {
    fn scroll(&mut self, scroll: f32) {
        self.rel_adr = if scroll.is_sign_positive() {
            self.rel_adr.saturating_sub(scroll as u32 * 4)
        } else {
            self.rel_adr.saturating_add(-scroll as u32 * 4)
        };
    }
}

impl DebugGui for Disasm {
    type State = State;
    type Conf = Conf;

    fn draw(
        &mut self,
        ui: &imgui::Ui,
        io: &imgui::Io,
        global_state: &DebugGlobalState,
        state: &Self::State,
        conf: &mut Self::Conf,
    ) {
        if self.hovered {
            let wheel = io.mouse_wheel;
            self.scroll(wheel);
        }
        if ui.button("Go to PC") {
            if let Some((section, rel)) = MemorySection::from_adr(global_state.arm9_pc()) {
                self.mem_section = section;
                self.rel_adr = rel;
            }
        }
        let (start, end) = self.mem_section.start_and_len();
        ui.child_window("disasm")
            .border(true)
            .scroll_bar(false)
            .scrollable(false)
            .size([-20.0, 0.0])
            .build(|| {
                let column_height = ui.current_font_size() + 2.0;
                let [_, window_height] = ui.content_region_max();
                let tmp = ("".to_string(), Box::<[u8]>::default());
                self.line_cnt = ((window_height / column_height) as usize)
                    .clamp(1, end.saturating_sub(self.rel_adr as usize));
                self.abs_adr = start + self.rel_adr;
                let _table = ui
                    .begin_table_with_flags("disasm_tbl", 3, TableFlags::SIZING_FIXED_FIT)
                    .unwrap();
                for i in 0..self.line_cnt {
                    let (str, bytes) = if let Some(val) = state.instr.get(i) {
                        (&val.print, &val.bytes)
                    } else {
                        (&tmp.0, &tmp.1)
                    };
                    ui.table_next_row_with_height(TableRowFlags::empty(), column_height);
                    ui.table_next_column();
                    let adr = self.abs_adr + i as u32 * 4;
                    if adr == global_state.arm9_pc() {
                        ui.table_set_bg_color(TableBgTarget::ROW_BG0, [0.45, 0.45, 0.45, 0.5]);
                    }
                    ui.text(format!("{adr:08X}:"));
                    ui.table_next_column();
                    ui.text_colored([0.4, 0.4, 0.4, 1.0], format!("{bytes:02X?}"));
                    ui.table_next_column();
                    ui.text_colored([0.0, 0.6, 0.0, 1.0], str);
                }
            });
        self.hovered = ui.is_item_hovered();
        ui.same_line();
        ui.child_window("scroll")
            .scrollable(true)
            .scroll_bar(true)
            .always_vertical_scrollbar(true)
            .build(|| {});

        conf.line = self.abs_adr;
        conf.count = self.line_cnt;
    }
}
