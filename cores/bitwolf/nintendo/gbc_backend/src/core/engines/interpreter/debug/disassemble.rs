use super::{registers::R16, Emu, Interpreter};
use crate::core::{
    bus,
    cpu::{
        instructions::{
            decode::{Bit, RSTVec, CC},
            Prefixed, Unprefixed,
        },
        registers::R8,
    },
};
use common_core::disassembly::DisassembledOutput;
use std::fmt::Display;

pub enum ControlFlow {
    Jmp(u64),
    Call(u64),
    Nop,
    Stop,
}

pub struct Output {
    pub output: DisassembledOutput,
    pub ctrl_flow: Option<ControlFlow>,
}

pub fn disassemble(emu: &Emu<Interpreter>, adr: u16) -> Output {
    let opc = bus::debug::read::read(&emu.bus, adr);
    let imm0 = bus::debug::read::read(&emu.bus, adr.wrapping_add(1));
    let imm1 = bus::debug::read::read(&emu.bus, adr.wrapping_add(2));
    let instr = Unprefixed::from_u8(opc);
    let b1 = vec![opc];
    let b2 = vec![opc, imm0];
    let b3 = vec![opc, imm0, imm1];
    let rel_adr = (adr as u64)
        .wrapping_add_signed(imm0 as i8 as i64)
        .wrapping_add(2);
    let abs_adr = ((imm1 as u64) << 8) | (imm0 as u64);
    let comment = None;
    let label = |adr: u64| -> String { format!("L_{adr:04X}") };
    match instr {
        Unprefixed::NOP => Output {
            output: DisassembledOutput::Instr {
                string_repr: "nop".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: Some(ControlFlow::Nop),
        },
        Unprefixed::STOP => Output {
            output: DisassembledOutput::Instr {
                string_repr: "stop".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::RLCA => Output {
            output: DisassembledOutput::Instr {
                string_repr: "rlca".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::RRCA => Output {
            output: DisassembledOutput::Instr {
                string_repr: "rrca".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::RLA => Output {
            output: DisassembledOutput::Instr {
                string_repr: "rla".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::RRA => Output {
            output: DisassembledOutput::Instr {
                string_repr: "rra".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::DAA => Output {
            output: DisassembledOutput::Instr {
                string_repr: "daa".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::CPL => Output {
            output: DisassembledOutput::Instr {
                string_repr: "cpl".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::SCF => Output {
            output: DisassembledOutput::Instr {
                string_repr: "scf".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::CCF => Output {
            output: DisassembledOutput::Instr {
                string_repr: "ccf".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::JR => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("jr {}", label(rel_adr)),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: Some(ControlFlow::Jmp(rel_adr)),
        },
        Unprefixed::HALT => Output {
            output: DisassembledOutput::Instr {
                string_repr: "halt".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::RET => Output {
            output: DisassembledOutput::Instr {
                string_repr: "ret".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: Some(ControlFlow::Stop),
        },
        Unprefixed::RETI => Output {
            output: DisassembledOutput::Instr {
                string_repr: "reti".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: Some(ControlFlow::Stop),
        },
        Unprefixed::JPHL => Output {
            output: DisassembledOutput::Instr {
                string_repr: "jp hl".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: Some(ControlFlow::Stop),
        },
        Unprefixed::JP => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("jp {}", label(abs_adr)),
                byte_repr: b3,
                comment,
            },
            ctrl_flow: Some(ControlFlow::Jmp(abs_adr)),
        },
        Unprefixed::DI => Output {
            output: DisassembledOutput::Instr {
                string_repr: "di".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::EI => Output {
            output: DisassembledOutput::Instr {
                string_repr: "ei".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::CALL => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("call {}", label(abs_adr)),
                byte_repr: b3,
                comment,
            },
            ctrl_flow: Some(ControlFlow::Call(abs_adr)),
        },
        Unprefixed::ADD_SP_I => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("add sp, {}", imm0 as i8),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::CB => Output {
            output: DisassembledOutput::Instr {
                string_repr: disassemble_prefixed(Prefixed::from_u8(imm0)),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::RST(rst) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("rst {rst}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::PUSH(r16) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("push {r16}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::POP(r16) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("pop {r16}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::CALLCC(cc) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("call{cc} {}", label(abs_adr)),
                byte_repr: b3,
                comment,
            },
            ctrl_flow: Some(ControlFlow::Call(abs_adr)),
        },
        Unprefixed::JPCC(cc) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("jp{cc} {}", label(abs_adr)),
                byte_repr: b3,
                comment,
            },
            ctrl_flow: Some(ControlFlow::Call(abs_adr)),
        },
        Unprefixed::RETCC(cc) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ret{cc}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::JRCC(cc) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("jr{cc} {}", label(rel_adr)),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: Some(ControlFlow::Call(rel_adr)),
        },
        Unprefixed::ADD_HL_R16(r16) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("add hl, {r16}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::ADD_HL_SP => Output {
            output: DisassembledOutput::Instr {
                string_repr: "add hl, sp".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_PNN_SP => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ld [0x{abs_adr:04X}], sp"),
                byte_repr: b3,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_PHLI_A => Output {
            output: DisassembledOutput::Instr {
                string_repr: "ld [hl+], a".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_PHLD_A => Output {
            output: DisassembledOutput::Instr {
                string_repr: "ld [hl-], a".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LDH_A_PN => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ldh a, [0x{imm0:02X}]"),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LDH_PN_A => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ldh [0x{imm0:02X}], a"),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LDH_A_PC => Output {
            output: DisassembledOutput::Instr {
                string_repr: "ldh a, [c]".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LDH_PC_A => Output {
            output: DisassembledOutput::Instr {
                string_repr: "ldh [c], a".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_A_PHLI => Output {
            output: DisassembledOutput::Instr {
                string_repr: "ld a, [hl+]".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_A_PHLD => Output {
            output: DisassembledOutput::Instr {
                string_repr: "ld a, [hl-]".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_R8_R8(dst, src) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ld {dst}, {src}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_R8_PHL(dst) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ld {dst}, [hl]"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_PHL_R8(src) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ld [hl], {src}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_R8_N(dst) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ld {dst}, 0x{imm0:02X}"),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_PHL_N => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ld [hl], 0x{imm0:02X}"),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_R16_NN(dst) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ld {dst}, 0x{abs_adr:04X}"),
                byte_repr: b3,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_SP_NN => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ld sp, 0x{abs_adr:04X}"),
                byte_repr: b3,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_PR16_A(dst) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ld [{dst}], a"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_A_PR16(src) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ld a, [{src}]"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_PNN_A => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ld [0x{abs_adr:04X}], a"),
                byte_repr: b3,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_A_PNN => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ld a, [0x{abs_adr:04X}]"),
                byte_repr: b3,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_HL_SP_I => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ld hl, sp, {}", imm0 as i8),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::LD_SP_HL => Output {
            output: DisassembledOutput::Instr {
                string_repr: "ld sp, hl".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::INC_R8(dst) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("inc {dst}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::INC_PHL => Output {
            output: DisassembledOutput::Instr {
                string_repr: "inc [hl]".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::INC_R16(dst) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("inc {dst}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::INC_SP => Output {
            output: DisassembledOutput::Instr {
                string_repr: "inc sp".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::DEC_R8(dst) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("dec {dst}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::DEC_PHL => Output {
            output: DisassembledOutput::Instr {
                string_repr: "dec [hl]".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::DEC_R16(dst) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("dec {dst}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::DEC_SP => Output {
            output: DisassembledOutput::Instr {
                string_repr: "dec sp".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::ADD_N => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("add 0x{imm0:02X}"),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::ADD_R8(src) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("add {src}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::ADD_PHL => Output {
            output: DisassembledOutput::Instr {
                string_repr: "add [hl]".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::ADC_N => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("adc 0x{imm0:02X}"),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::ADC_R8(src) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("adc {src}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::ADC_PHL => Output {
            output: DisassembledOutput::Instr {
                string_repr: "adc [hl]".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::SUB_N => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("sub 0x{imm0:02X}"),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::SUB_R8(src) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("sub {src}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::SUB_PHL => Output {
            output: DisassembledOutput::Instr {
                string_repr: "sub [hl]".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::SBC_N => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("sbc 0x{imm0:02X}"),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::SBC_R8(src) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("sbc {src}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::SBC_PHL => Output {
            output: DisassembledOutput::Instr {
                string_repr: "sbc [hl]".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::AND_N => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("and 0x{imm0:02X}"),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::AND_R8(src) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("ad {src}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::AND_PHL => Output {
            output: DisassembledOutput::Instr {
                string_repr: "and [hl]".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::XOR_N => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("xor 0x{imm0:02X}"),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::XOR_R8(src) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("xor {src}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::XOR_PHL => Output {
            output: DisassembledOutput::Instr {
                string_repr: "xor [hl]".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::OR_N => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("or 0x{imm0:02X}"),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::OR_R8(src) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("or {src}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::OR_PHL => Output {
            output: DisassembledOutput::Instr {
                string_repr: "or [hl]".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::CP_N => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("cp 0x{imm0:02X}"),
                byte_repr: b2,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::CP_R8(src) => Output {
            output: DisassembledOutput::Instr {
                string_repr: format!("cp {src}"),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::CP_PHL => Output {
            output: DisassembledOutput::Instr {
                string_repr: "cp [hl]".to_string(),
                byte_repr: b1,
                comment,
            },
            ctrl_flow: None,
        },
        Unprefixed::INVALID => Output {
            output: DisassembledOutput::Data { data: opc },
            ctrl_flow: None,
        },
    }
}

fn disassemble_prefixed(prefixed: Prefixed) -> String {
    match prefixed {
        Prefixed::RLC(r8) => format!("rlc {r8}"),
        Prefixed::RLC_PHL => format!("rlc [hl]"),
        Prefixed::RRC(r8) => format!("rrc {r8}"),
        Prefixed::RRC_PHL => format!("rrc [hl]"),
        Prefixed::RL(r8) => format!("rl {r8}"),
        Prefixed::RL_PHL => format!("rl [hl]"),
        Prefixed::RR(r8) => format!("rr {r8}"),
        Prefixed::RR_PHL => format!("rr [hl]"),
        Prefixed::SLA(r8) => format!("sla {r8}"),
        Prefixed::SLA_PHL => format!("sla [hl]"),
        Prefixed::SRA(r8) => format!("sra {r8}"),
        Prefixed::SRA_PHL => format!("sra [hl]"),
        Prefixed::SWAP(r8) => format!("swap {r8}"),
        Prefixed::SWAP_PHL => format!("swap [hl]"),
        Prefixed::SRL(r8) => format!("srl {r8}"),
        Prefixed::SRL_PHL => format!("srl [hl]"),
        Prefixed::BIT(b, r8) => format!("bit {b}, {r8}"),
        Prefixed::BIT_PHL(b) => format!("bit {b}, [hl]"),
        Prefixed::RES(b, r8) => format!("res {b}, {r8}"),
        Prefixed::RES_PHL(b) => format!("res {b}, [hl]"),
        Prefixed::SET(b, r8) => format!("set {b}, {r8}"),
        Prefixed::SET_PHL(b) => format!("set {b}, [hl]"),
    }
}

impl Display for CC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CC::NZ => "nz",
            CC::Z => "z",
            CC::NC => "nc",
            CC::C => "c",
        })
    }
}

impl Display for R16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            R16::AF => "af",
            R16::BC => "bc",
            R16::DE => "de",
            R16::HL => "hl",
        })
    }
}

impl Display for R8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            R8::A => "a",
            R8::B => "b",
            R8::C => "c",
            R8::D => "d",
            R8::E => "e",
            R8::H => "h",
            R8::L => "l",
        })
    }
}

impl Display for RSTVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("vec_{:02X}", *self as u8))
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Bit::B0 => "0",
            Bit::B1 => "1",
            Bit::B2 => "2",
            Bit::B3 => "3",
            Bit::B4 => "4",
            Bit::B5 => "5",
            Bit::B6 => "6",
            Bit::B7 => "7",
        })
    }
}
