pub mod substates {
    use std::collections::HashSet;

    #[derive(Default)]
    pub struct Control {
        pub running: bool,
        pub break_points: HashSet<u16>,
    }
}

#[derive(Default)]
pub struct State {
    pub ctrl: substates::Control,
}
