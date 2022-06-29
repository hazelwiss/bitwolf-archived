pub enum DisassembledOutput {
    Instr {
        string_repr: String,
        byte_repr: Vec<u8>,
        comment: Option<String>,
    },
    Data {
        data: Vec<u8>,
        comment: Option<String>,
    },
}
