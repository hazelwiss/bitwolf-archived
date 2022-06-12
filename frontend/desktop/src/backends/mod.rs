pub mod nintendo;

pub enum Backend {
    None,
    NintendoGB(nintendo::gb::Context),
}
