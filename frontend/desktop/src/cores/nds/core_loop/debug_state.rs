use nds_core::rom::CartridgeHeader;

#[derive(Default)]
pub(crate) struct DebugState {
    cartidge_header: CartridgeHeader,
}
