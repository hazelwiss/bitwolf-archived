mod defs;

use super::Interpreter;
use cpu::instrutions::{
    decode::{ALUArg, RPTblEntry, ALU, DEC, E8, INC, LD},
    Unprefixed,
};

impl Interpreter {
    pub fn fetch_decode_execute(&mut self) {
        let val = Unprefixed::from_byte(0 as u8);
        match val {
            Unprefixed::NOP => self.nop(),
            Unprefixed::STOP => self.stop(),
            Unprefixed::RLCA => self.rlca(),
            Unprefixed::RRCA => self.rrca(),
            Unprefixed::RLA => self.rla(),
            Unprefixed::RRA => self.rra(),
            Unprefixed::DAA => self.daa(),
            Unprefixed::CPL => self.cpl(),
            Unprefixed::SCF => self.scf(),
            Unprefixed::CCF => self.ccf(),
            Unprefixed::JR => self.jr(),
            Unprefixed::HALT => self.halt(),
            Unprefixed::RET => self.ret(),
            Unprefixed::RETI => self.reti(),
            Unprefixed::JPHL => self.jp_hl(),
            Unprefixed::JP => self.jp(),
            Unprefixed::DI => self.di(),
            Unprefixed::EI => self.ei(),
            Unprefixed::CALL => self.call(),
            Unprefixed::ADDSP => self.add_sp_e8(),
            Unprefixed::CB => self.nop(), // temporary
            Unprefixed::RST(vec) => self.rst(vec),
            Unprefixed::PUSH(src) => match src {
                other => self.push_r16(other),
            },
            Unprefixed::POP(dst) => match dst {
                cpu::registers::R16::AF => self.pop_af(),
                other => self.pop_r16(other),
            },
            Unprefixed::CALLCC(cc) => self.call_cc(cc),
            Unprefixed::JPCC(cc) => self.jp_cc(cc),
            Unprefixed::RETCC(cc) => self.ret_cc(cc),
            Unprefixed::ADDHL(src) => match src {
                RPTblEntry::R16(src) => self.add_hl_r16(src),
                RPTblEntry::SP => self.add_hl_sp(),
            },
            Unprefixed::JRCC(cc) => self.jr_cc(cc),
            Unprefixed::LD(ld) => match ld {
                LD::PNN_SP => todo!(),
                LD::PHLI_A => todo!(),
                LD::PHLD_A => todo!(),
                LD::H_A_PN => todo!(),
                LD::H_PN_A => todo!(),
                LD::H_A_PC => todo!(),
                LD::H_PC_A => todo!(),
                LD::A_PHLI => todo!(),
                LD::A_PHLD => todo!(),
                LD::E8_R8(_, _) => todo!(),
                LD::E8_N(_) => todo!(),
                LD::R16_NN(_) => todo!(),
                LD::PR16_R8(_, _) => todo!(),
                LD::R8_PR16(_, _) => todo!(),
                LD::PNN_A => todo!(),
                LD::A_PNN => todo!(),
                LD::HL_SP_D => todo!(),
                LD::SP_HL => todo!(),
            },
            Unprefixed::INC(inc) => match inc {
                INC::E8(e8) => match e8 {
                    E8::R8(r8) => self.inc_r8(r8),
                    E8::PHL => self.inc_phl(),
                },
                INC::RPTblRet(v) => match v {
                    RPTblEntry::R16(r16) => self.inc_r16(r16),
                    RPTblEntry::SP => self.inc_sp(),
                },
            },
            Unprefixed::DEC(dec) => match dec {
                DEC::E8(e8) => match e8 {
                    E8::R8(dst) => self.dec_r8(dst),
                    E8::PHL => self.dec_phl(),
                },
                DEC::RPTblRet(v) => match v {
                    RPTblEntry::R16(dst) => self.dec_r16(dst),
                    RPTblEntry::SP => self.dec_sp(),
                },
            },
            Unprefixed::ALU(alu) => match alu {
                ALU::ADD(src) => match src {
                    ALUArg::E8(e8) => match e8 {
                        E8::R8(src) => self.add_r8(src),
                        E8::PHL => self.add_phl(),
                    },
                    ALUArg::N => self.add_n8(),
                },
                ALU::ADC(src) => match src {
                    ALUArg::E8(e8) => match e8 {
                        E8::R8(src) => self.adc_r8(src),
                        E8::PHL => self.adc_phl(),
                    },
                    ALUArg::N => self.adc_n8(),
                },
                ALU::SUB(src) => match src {
                    ALUArg::E8(e8) => match e8 {
                        E8::R8(src) => self.sub_r8(src),
                        E8::PHL => self.sub_phl(),
                    },
                    ALUArg::N => self.sub_n8(),
                },
                ALU::SBC(src) => match src {
                    ALUArg::E8(e8) => match e8 {
                        E8::R8(src) => self.sbc_r8(src),
                        E8::PHL => self.sbc_phl(),
                    },
                    ALUArg::N => self.sbc_n8(),
                },
                ALU::AND(src) => match src {
                    ALUArg::E8(e8) => match e8 {
                        E8::R8(src) => self.and_r8(src),
                        E8::PHL => self.and_phl(),
                    },
                    ALUArg::N => self.and_n8(),
                },
                ALU::XOR(src) => match src {
                    ALUArg::E8(e8) => match e8 {
                        E8::R8(src) => self.xor_r8(src),
                        E8::PHL => self.xor_phl(),
                    },
                    ALUArg::N => self.xor_n8(),
                },
                ALU::OR(src) => match src {
                    ALUArg::E8(e8) => match e8 {
                        E8::R8(src) => self.or_r8(src),
                        E8::PHL => self.or_phl(),
                    },
                    ALUArg::N => self.or_n8(),
                },
                ALU::CP(src) => match src {
                    ALUArg::E8(e8) => match e8 {
                        E8::R8(src) => self.cp_r8(src),
                        E8::PHL => self.cp_phl(),
                    },
                    ALUArg::N => self.cp_n8(),
                },
            },
            Unprefixed::INVALID => logger::fatal!("Attempted to executed invalid instruction"),
        }
    }
}
