use crate::core::{
    cpu::{instructions::Unprefixed, CPU},
    cycles::Cycles,
};
use crate::{engines::Engine, Core};

pub struct Interpreter;

impl Engine for Interpreter {
    type EngineData = ();
}

pub fn step(core: &mut Core<Interpreter>) {
    core.cpu.interrupt_handler();
    fetch_decode_execute(&mut core.cpu);
}

fn fetch_decode_execute(cpu: &mut CPU<Interpreter>) {
    if cpu.halted_get() {
        cpu.tick(Cycles::M(1));
    } else {
        let val = Unprefixed::from_u8(cpu.fetch());
        match val {
            Unprefixed::NOP => cpu.nop(),
            Unprefixed::STOP => cpu.stop(),
            Unprefixed::RLCA => cpu.rlca(),
            Unprefixed::RRCA => cpu.rrca(),
            Unprefixed::RLA => cpu.rla(),
            Unprefixed::RRA => cpu.rra(),
            Unprefixed::DAA => cpu.daa(),
            Unprefixed::CPL => cpu.cpl(),
            Unprefixed::SCF => cpu.scf(),
            Unprefixed::CCF => cpu.ccf(),
            Unprefixed::JR => cpu.jr(),
            Unprefixed::HALT => cpu.halt(),
            Unprefixed::RET => cpu.ret(),
            Unprefixed::RETI => cpu.reti(),
            Unprefixed::JPHL => cpu.jp_hl(),
            Unprefixed::JP => cpu.jp(),
            Unprefixed::DI => cpu.di(),
            Unprefixed::EI => cpu.ei(),
            Unprefixed::CALL => cpu.call(),
            Unprefixed::ADD_SP_I => cpu.add_sp_e8(),
            Unprefixed::CB => cpu.cb(),
            Unprefixed::RST(vec) => cpu.rst(vec),
            Unprefixed::PUSH(src) => cpu.push_r16(src),
            Unprefixed::POP(dst) => cpu.pop_r16(dst),
            Unprefixed::CALLCC(cc) => cpu.call_cc(cc),
            Unprefixed::JPCC(cc) => cpu.jp_cc(cc),
            Unprefixed::RETCC(cc) => cpu.ret_cc(cc),
            Unprefixed::JRCC(cc) => cpu.jr_cc(cc),
            Unprefixed::ADD_HL_R16(src) => cpu.add_hl_r16(src),
            Unprefixed::ADD_HL_SP => cpu.add_hl_sp(),
            Unprefixed::LD_PNN_SP => cpu.ld_pn16_sp(),
            Unprefixed::LD_PHLI_A => cpu.ld_phli_a(),
            Unprefixed::LD_PHLD_A => cpu.ld_phld_a(),
            Unprefixed::LDH_A_PN => cpu.ldh_a_pn8(),
            Unprefixed::LDH_PN_A => cpu.ldh_pn8_a(),
            Unprefixed::LDH_A_PC => cpu.ldh_a_pc(),
            Unprefixed::LDH_PC_A => cpu.ldh_pc_a(),
            Unprefixed::LD_A_PHLI => cpu.ld_a_phli(),
            Unprefixed::LD_A_PHLD => cpu.ld_a_phld(),
            Unprefixed::LD_R8_R8(dst, src) => cpu.ld_r8_r8(dst, src),
            Unprefixed::LD_R8_PHL(dst) => cpu.ld_r8_phl(dst),
            Unprefixed::LD_PHL_R8(src) => cpu.ld_phl_r8(src),
            Unprefixed::LD_R8_N(dst) => cpu.ld_r8_n8(dst),
            Unprefixed::LD_PHL_N => cpu.ld_phl_n8(),
            Unprefixed::LD_R16_NN(dst) => cpu.ld_r16_n16(dst),
            Unprefixed::LD_SP_NN => cpu.ld_sp_n16(),
            Unprefixed::LD_PR16_A(dst) => cpu.ld_pr16_a(dst),
            Unprefixed::LD_A_PR16(src) => cpu.ld_a_pr16(src),
            Unprefixed::LD_PNN_A => cpu.ld_pn16_a(),
            Unprefixed::LD_A_PNN => cpu.ld_a_pn16(),
            Unprefixed::LD_HL_SP_I => cpu.ld_hl_sp_e8(),
            Unprefixed::LD_SP_HL => cpu.ld_sp_hl(),
            Unprefixed::INC_R8(dst) => cpu.inc_r8(dst),
            Unprefixed::INC_PHL => cpu.inc_phl(),
            Unprefixed::INC_R16(dst) => cpu.inc_r16(dst),
            Unprefixed::INC_SP => cpu.inc_sp(),
            Unprefixed::DEC_R8(dst) => cpu.dec_r8(dst),
            Unprefixed::DEC_PHL => cpu.dec_phl(),
            Unprefixed::DEC_R16(dst) => cpu.dec_r16(dst),
            Unprefixed::DEC_SP => cpu.dec_sp(),
            Unprefixed::ADD_N => cpu.add_n8(),
            Unprefixed::ADD_R8(src) => cpu.add_r8(src),
            Unprefixed::ADD_PHL => cpu.add_phl(),
            Unprefixed::ADC_N => cpu.adc_n8(),
            Unprefixed::ADC_R8(src) => cpu.adc_r8(src),
            Unprefixed::ADC_PHL => cpu.adc_phl(),
            Unprefixed::SUB_N => cpu.sub_n8(),
            Unprefixed::SUB_R8(src) => cpu.sub_r8(src),
            Unprefixed::SUB_PHL => cpu.sub_phl(),
            Unprefixed::SBC_N => cpu.sbc_n8(),
            Unprefixed::SBC_R8(src) => cpu.sbc_r8(src),
            Unprefixed::SBC_PHL => cpu.sbc_phl(),
            Unprefixed::AND_N => cpu.and_n8(),
            Unprefixed::AND_R8(src) => cpu.and_r8(src),
            Unprefixed::AND_PHL => cpu.and_phl(),
            Unprefixed::XOR_N => cpu.xor_n8(),
            Unprefixed::XOR_R8(src) => cpu.xor_r8(src),
            Unprefixed::XOR_PHL => cpu.xor_phl(),
            Unprefixed::OR_N => cpu.or_n8(),
            Unprefixed::OR_R8(src) => cpu.or_r8(src),
            Unprefixed::OR_PHL => cpu.or_phl(),
            Unprefixed::CP_N => cpu.cp_n8(),
            Unprefixed::CP_R8(src) => cpu.cp_r8(src),
            Unprefixed::CP_PHL => cpu.cp_phl(),
            Unprefixed::INVALID => logger::fatal!("Attempted to executed invalid instruction"),
        }
    }
}
