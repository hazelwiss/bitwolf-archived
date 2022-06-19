use crate::engines::interpreter::Interpreter;
use cpu::{
    instrutions::decode::RSTVec,
    instrutions::decode::CC,
    registers::{R16, R8},
};

impl Interpreter {
    pub fn adc_r8<const SRC: R8>(&mut self) {}

    pub fn adc_phl(&mut self) {}

    pub fn adc_n8(&mut self) {}

    pub fn add_r8<const SRC: R8>(&mut self) {}

    pub fn add_phl(&mut self) {}

    pub fn add_n8(&mut self) {}

    pub fn and_r8<const SRC: R8>(&mut self) {}

    pub fn and_phl(&mut self) {}

    pub fn and_n8(&mut self) {}

    pub fn cp_r8<const SRC: R8>(&mut self) {}

    pub fn cp_phl(&mut self) {}

    pub fn cp_n8(&mut self) {}

    pub fn dec_r8<const DST: R8>(&mut self) {}

    pub fn dec_phl(&mut self) {}

    pub fn inc_r8<const DST: R8>(&mut self) {}

    pub fn inc_phl(&mut self) {}

    pub fn or_r8<const SRC: R8>(&mut self) {}

    pub fn or_phl(&mut self) {}

    pub fn or_n8(&mut self) {}

    pub fn sbc_r8<const SRC: R8>(&mut self) {}

    pub fn sbc_phl(&mut self) {}

    pub fn sbc_n8(&mut self) {}

    pub fn sub_r8<const SRC: R8>(&mut self) {}

    pub fn sub_phl(&mut self) {}

    pub fn sub_n8(&mut self) {}

    pub fn xor_r8<const SRC: R8>(&mut self) {}

    pub fn xor_phl(&mut self) {}

    pub fn xor_n8(&mut self) {}

    pub fn add_hl_r16<const SRC: R16>(&mut self) {}

    pub fn dec_r16<const DST: R16>(&mut self) {}

    pub fn inc_r16<const DST: R16>(&mut self) {}

    pub fn bit_r8<const SRC: R8>(&mut self) {}

    pub fn bit_phl<const SRC: R8>(&mut self) {}

    pub fn res_r8<const DST: R8>(&mut self) {}

    pub fn res_phl(&mut self) {}

    pub fn set_r8<const DST: R8>(&mut self) {}

    pub fn set_phl(&mut self) {}

    pub fn swap_r8<const DST: R8>(&mut self) {}

    pub fn swap_phl(&mut self) {}

    pub fn rl_r8<const SRC: R8>(&mut self) {}

    pub fn rl_phl(&mut self) {}

    pub fn rla(&mut self) {}

    pub fn rlc_r8<const SRC: R8>(&mut self) {}

    pub fn rlc_phl(&mut self) {}

    pub fn rlca(&mut self) {}

    pub fn rr_r8<const SRC: R8>(&mut self) {}

    pub fn rr_phl(&mut self) {}

    pub fn rra(&mut self) {}

    pub fn rrc_r8<const SRC: R8>(&mut self) {}

    pub fn rrc_phl(&mut self) {}

    pub fn rrca(&mut self) {}

    pub fn sla_r8<const SRC: R8>(&mut self) {}

    pub fn sla_phl(&mut self) {}

    pub fn sra_r8<const SRC: R8>(&mut self) {}

    pub fn sra_phl(&mut self) {}

    pub fn srl_r8<const SRC: R8>(&mut self) {}

    pub fn srl_phl(&mut self) {}

    pub fn ld_r8_r8<const DST: R8, const SRC: R8>(&mut self) {}

    pub fn ld_r8_n8<const DST: R8>(&mut self) {}

    pub fn ld_r16_n16<const DST: R16>(&mut self) {}

    pub fn ld_phl_r8<const SRC: R8>(&mut self) {}

    pub fn ld_phl_n8(&mut self) {}

    pub fn ld_r8_phl<const DST: R8>(&mut self) {}

    pub fn ld_pr16_a(&mut self) {}

    pub fn ld_pn16_a(&mut self) {}

    pub fn ldh_pc_a(&mut self) {}

    pub fn ld_a_pr16(&mut self) {}

    pub fn ld_a_pn16(&mut self) {}

    pub fn ldh_a_pn8(&mut self) {}

    pub fn ldh_a_pc(&mut self) {}

    pub fn ld_phli_a(&mut self) {}

    pub fn ld_phld_a(&mut self) {}

    pub fn ld_a_hli(&mut self) {}

    pub fn ld_a_hld(&mut self) {}

    pub fn call(&mut self) {}

    pub fn call_cc<const cc: CC>(&mut self) {}

    pub fn jp_hl(&mut self) {}

    pub fn jp(&mut self) {}

    pub fn jp_cc<const cc: CC>(&mut self) {}

    pub fn jr(&mut self) {}

    pub fn jr_cc<const cc: CC>(&mut self) {}

    pub fn ret_cc<const cc: CC>(&mut self) {}

    pub fn ret(&mut self) {}

    pub fn reti(&mut self) {}

    pub fn rst<const vec: RSTVec>(&mut self) {}

    pub fn add_hl_sp(&mut self) {}

    pub fn add_sp_e8(&mut self) {}

    pub fn dec_sp(&mut self) {}

    pub fn inc_sp(&mut self) {}

    pub fn ld_sp_n16(&mut self) {}

    pub fn ld_pn16_sp(&mut self) {}

    pub fn ld_hl_sp_e8(&mut self) {}

    pub fn ld_sp_hl(&mut self) {}

    pub fn pop_af(&mut self) {}

    pub fn pop_r16<const DST: R16>(&mut self) {}

    pub fn push_r16<const SRC: R16>(&mut self) {}

    pub fn ccf(&mut self) {}

    pub fn cpl(&mut self) {}

    pub fn daa(&mut self) {}

    pub fn di(&mut self) {}

    pub fn ei(&mut self) {}

    pub fn halt(&mut self) {}

    pub fn nop(&mut self) {}

    pub fn scf(&mut self) {}

    pub fn stop(&mut self) {}
}
