/// Decoded ARM9 opcode.
pub enum ArmOpcode {
    /// Data processing immediate shift
    DPIS,
    /// Miscellaneous instructions
    MI,
    /// Data processing register shift
    DPRS,
    /// Multiples, exta load/stores
    Mul,
    /// Data processing immediate
    DPI,
    /// Undefined instruction
    Undef,
    /// Move immediate to status register
    MISR,
    /// Load/store immediate offset
    LSIO,
    /// Load/store register offset
    LSRO,
    /// Load/store multiple
    LSM,
    /// Branch and branch with link
    BBL,
    /// Branch and branch with link and change to Thumb
    BX,
    /// Coprocessor load/store and double register transfers
    CLSDRT,
    /// Coprocessor data processing
    CDP,
    /// Coprocessor register transfer
    CRT,
    /// Software interrupt
    SWI,
}
