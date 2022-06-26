use super::Interpreter;
use crate::{
    cpu::{
        instrutions::{
            decode::{Bit, RSTVec, CC},
            Prefixed,
        },
        registers::{Flag, R16, R8},
    },
    cycles::Cycles,
    emu::event_slots::Slot,
    Emu,
};

impl Emu<Interpreter> {
    pub(crate) fn adc_r8(&mut self, src: R8) {
        self.alu_adc(self.r8_get(src), self.flag_get(Flag::C));
    }

    pub(crate) fn adc_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_adc(phl, self.flag_get(Flag::C));
    }

    pub(crate) fn adc_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_adc(fetch, self.flag_get(Flag::C));
    }

    pub(crate) fn add_r8(&mut self, src: R8) {
        self.alu_adc(self.r8_get(src), false);
    }

    pub(crate) fn add_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_adc(phl, false);
    }

    pub(crate) fn add_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_adc(fetch, false);
    }

    pub(crate) fn and_r8(&mut self, src: R8) {
        self.alu_and(self.r8_get(src));
    }

    pub(crate) fn and_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_and(phl);
    }

    pub(crate) fn and_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_and(fetch);
    }

    pub(crate) fn cp_r8(&mut self, src: R8) {
        self.alu_cp(self.r8_get(src));
    }

    pub(crate) fn cp_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_cp(phl);
    }

    pub(crate) fn cp_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_cp(fetch);
    }

    pub(crate) fn dec_r8(&mut self, dst: R8) {
        let result = self.generic_dec(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub(crate) fn dec_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_dec(phl);
        self.phl_set(result);
    }

    pub(crate) fn inc_r8(&mut self, dst: R8) {
        let result = self.generic_inc(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub(crate) fn inc_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_inc(phl);
        self.phl_set(result);
    }

    pub(crate) fn or_r8(&mut self, src: R8) {
        self.alu_or(self.r8_get(src));
    }

    pub(crate) fn or_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_or(phl);
    }

    pub(crate) fn or_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_or(fetch);
    }

    pub(crate) fn sbc_r8(&mut self, src: R8) {
        self.alu_sbc(self.r8_get(src), self.flag_get(Flag::C));
    }

    pub(crate) fn sbc_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_sbc(phl, self.flag_get(Flag::C));
    }

    pub(crate) fn sbc_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_sbc(fetch, self.flag_get(Flag::C));
    }

    pub(crate) fn sub_r8(&mut self, src: R8) {
        self.alu_sbc(self.r8_get(src), false);
    }

    pub(crate) fn sub_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_sbc(phl, false);
    }

    pub(crate) fn sub_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_sbc(fetch, false);
    }

    pub(crate) fn xor_r8(&mut self, src: R8) {
        self.alu_xor(self.r8_get(src));
    }

    pub(crate) fn xor_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_xor(phl);
    }

    pub(crate) fn xor_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_xor(fetch);
    }

    pub(crate) fn add_hl_r16(&mut self, src: R16) {
        let hl = self.r16_get(R16::HL) as u32;
        let src = self.r16_get(src) as u32;
        let result = hl + src;
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, (hl & 0x0FFF) + (src & 0x0FFF) > 0x0FFF);
        self.flag_set(Flag::C, result > 0xFFFF);
        self.r16_set(R16::HL, result as u16);
        self.tick(Cycles::M(1));
    }

    pub(crate) fn dec_r16(&mut self, dst: R16) {
        let r16 = self.r16_get(dst).wrapping_sub(1);
        self.r16_set(dst, r16);
    }

    pub(crate) fn inc_r16(&mut self, dst: R16) {
        let r16 = self.r16_get(dst).wrapping_add(1);
        self.r16_set(dst, r16);
    }

    pub(crate) fn bit_r8(&mut self, bit: Bit, src: R8) {
        self.generic_bit(bit, self.r8_get(src));
    }

    pub(crate) fn bit_phl(&mut self, bit: Bit) {
        let phl = self.phl_get();
        self.generic_bit(bit, phl);
    }

    pub(crate) fn res_r8(&mut self, bit: Bit, dst: R8) {
        let result = self.generic_res(bit, self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub(crate) fn res_phl(&mut self, bit: Bit) {
        let phl = self.phl_get();
        let result = self.generic_res(bit, phl);
        self.phl_set(result);
    }

    pub(crate) fn set_r8(&mut self, bit: Bit, dst: R8) {
        let result = self.generic_set(bit, self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub(crate) fn set_phl(&mut self, bit: Bit) {
        let phl = self.phl_get();
        let result = self.generic_set(bit, phl);
        self.phl_set(result);
    }

    pub(crate) fn swap_r8(&mut self, dst: R8) {
        let result = self.generic_swap(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub(crate) fn swap_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_swap(phl);
        self.phl_set(result);
    }

    pub(crate) fn rl_r8(&mut self, dst: R8) {
        let result = self.generic_rl(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub(crate) fn rl_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_rl(phl);
        self.phl_set(result);
    }

    pub(crate) fn rla(&mut self) {
        let result = self.generic_rl(self.r8_get(R8::A));
        self.r8_set(R8::A, result);
        self.flag_set(Flag::Z, false);
    }

    pub(crate) fn rlc_r8(&mut self, dst: R8) {
        let result = self.generic_rlc(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub(crate) fn rlc_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_rlc(phl);
        self.phl_set(result);
    }

    pub(crate) fn rlca(&mut self) {
        let result = self.generic_rlc(self.r8_get(R8::A));
        self.r8_set(R8::A, result);
        self.flag_set(Flag::Z, false);
    }

    pub(crate) fn rr_r8(&mut self, dst: R8) {
        let result = self.generic_rr(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub(crate) fn rr_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_rr(phl);
        self.phl_set(result);
    }

    pub(crate) fn rra(&mut self) {
        let result = self.generic_rr(self.r8_get(R8::A));
        self.r8_set(R8::A, result);
        self.flag_set(Flag::Z, false);
    }

    pub(crate) fn rrc_r8(&mut self, dst: R8) {
        let result = self.generic_rrc(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub(crate) fn rrc_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_rrc(phl);
        self.phl_set(result);
    }

    pub(crate) fn rrca(&mut self) {
        let result = self.generic_rrc(self.r8_get(R8::A));
        self.r8_set(R8::A, result);
        self.flag_set(Flag::Z, false);
    }

    pub(crate) fn sla_r8(&mut self, dst: R8) {
        let result = self.generic_sla(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub(crate) fn sla_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_sla(phl);
        self.phl_set(result);
    }

    pub(crate) fn sra_r8(&mut self, dst: R8) {
        let result = self.generic_sra(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub(crate) fn sra_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_sra(phl);
        self.phl_set(result);
    }

    pub(crate) fn srl_r8(&mut self, dst: R8) {
        let result = self.generic_srl(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub(crate) fn srl_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_srl(phl);
        self.phl_set(result);
    }

    pub(crate) fn ld_r8_r8(&mut self, dst: R8, src: R8) {
        let r8 = self.r8_get(src);
        self.r8_set(dst, r8);
    }

    pub(crate) fn ld_r8_n8(&mut self, dst: R8) {
        let fetch = self.fetch();
        self.r8_set(dst, fetch);
    }

    pub(crate) fn ld_r16_n16(&mut self, dst: R16) {
        let fetch = self.fetch16();
        self.r16_set(dst, fetch);
    }

    pub(crate) fn ld_phl_r8(&mut self, src: R8) {
        let r8 = self.r8_get(src);
        self.phl_set(r8);
    }

    pub(crate) fn ld_phl_n8(&mut self) {
        let fetch = self.fetch();
        self.phl_set(fetch);
    }

    pub(crate) fn ld_r8_phl(&mut self, dst: R8) {
        let phl = self.phl_get();
        self.r8_set(dst, phl);
    }

    pub(crate) fn ld_pr16_a(&mut self, dst: R16) {
        let ptr = self.r16_get(dst);
        let a = self.r8_get(R8::A);
        self.mem_write(ptr, a);
    }

    pub(crate) fn ld_pn16_a(&mut self) {
        let ptr = self.fetch16();
        let a = self.r8_get(R8::A);
        self.mem_write(ptr, a);
    }

    pub(crate) fn ldh_pn8_a(&mut self) {
        let ptr = 0xFF00 | self.fetch() as u16;
        let a = self.r8_get(R8::A);
        self.mem_write(ptr, a);
    }

    pub(crate) fn ldh_pc_a(&mut self) {
        let ptr = 0xFF00 | self.r8_get(R8::C) as u16;
        let a = self.r8_get(R8::A);
        self.mem_write(ptr, a);
    }

    pub(crate) fn ld_a_pr16(&mut self, src: R16) {
        let ptr = self.r16_get(src);
        let read = self.mem_read(ptr);
        self.r8_set(R8::A, read);
    }

    pub(crate) fn ld_a_pn16(&mut self) {
        let fetch = self.fetch16();
        let read = self.mem_read(fetch);
        self.r8_set(R8::A, read);
    }

    pub(crate) fn ldh_a_pn8(&mut self) {
        let adr = 0xFF00 | self.fetch() as u16;
        let read = self.mem_read(adr);
        self.r8_set(R8::A, read);
    }

    pub(crate) fn ldh_a_pc(&mut self) {
        let adr = 0xFF00 | self.r8_get(R8::C) as u16;
        let read = self.mem_read(adr);
        self.r8_set(R8::A, read);
    }

    pub(crate) fn ld_phli_a(&mut self) {
        let hl = self.r16_get(R16::HL);
        self.r16_set(R16::HL, hl.wrapping_add(1));
        let a = self.r8_get(R8::A);
        self.mem_write(hl, a);
    }

    pub(crate) fn ld_phld_a(&mut self) {
        let hl = self.r16_get(R16::HL);
        self.r16_set(R16::HL, hl.wrapping_sub(1));
        let a = self.r8_get(R8::A);
        self.mem_write(hl, a);
    }

    pub(crate) fn ld_a_phli(&mut self) {
        let hl = self.r16_get(R16::HL);
        self.r16_set(R16::HL, hl.wrapping_add(1));
        let read = self.mem_read(hl);
        self.r8_set(R8::A, read);
    }

    pub(crate) fn ld_a_phld(&mut self) {
        let hl = self.r16_get(R16::HL);
        self.r16_set(R16::HL, hl.wrapping_sub(1));
        let read = self.mem_read(hl);
        self.r8_set(R8::A, read);
    }

    pub(crate) fn call(&mut self) {
        let adr = self.fetch16();
        let pc = self.pc_get();
        self.push(pc);
        self.pc_set(adr);
        self.tick(Cycles::M(1));
    }

    pub(crate) fn call_cc(&mut self, cc: CC) {
        let adr = self.fetch16();
        if self.check_cond(cc) {
            let pc = self.pc_get();
            self.push(pc);
            self.pc_set(adr);
            self.tick(Cycles::M(1));
        }
    }

    pub(crate) fn jp_hl(&mut self) {
        let hl = self.r16_get(R16::HL);
        self.pc_set(hl);
    }

    pub(crate) fn jp(&mut self) {
        let adr = self.fetch16();
        self.pc_set(adr);
        self.tick(Cycles::M(1));
    }

    pub(crate) fn jp_cc(&mut self, cc: CC) {
        let adr = self.fetch16();
        if self.check_cond(cc) {
            self.pc_set(adr);
            self.tick(Cycles::M(1));
        }
    }

    pub(crate) fn jr(&mut self) {
        let imm = self.fetch() as i8 as i16;
        let pc = self.pc_get();
        self.pc_set(pc.wrapping_add_signed(imm) as u16);
        self.tick(Cycles::M(1));
    }

    pub(crate) fn jr_cc(&mut self, cc: CC) {
        let imm = self.fetch() as i8 as i16;
        if self.check_cond(cc) {
            let pc = self.pc_get();
            self.pc_set(pc.wrapping_add_signed(imm) as u16);
            self.tick(Cycles::M(1));
        }
    }

    pub(crate) fn ret_cc(&mut self, cc: CC) {
        if self.check_cond(cc) {
            self.ret()
        }
        self.tick(Cycles::M(1));
    }

    pub(crate) fn ret(&mut self) {
        let adr = self.pop();
        self.pc_set(adr);
        self.tick(Cycles::M(1));
    }

    pub(crate) fn reti(&mut self) {
        self.ret();
        self.ei();
    }

    pub(crate) fn rst(&mut self, vec: RSTVec) {
        let pc = self.pc_get();
        self.push(pc);
        self.pc_set(vec as u16);
        self.tick(Cycles::M(1));
    }

    pub(crate) fn add_hl_sp(&mut self) {
        let hl = self.r16_get(R16::HL) as u32;
        let sp = self.sp_get() as u32;
        let result = hl.wrapping_add(sp);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, (hl & 0x0FFF) + (sp & 0x0FFF) > 0x0FFF);
        self.flag_set(Flag::C, result > 0xFFFF);
        self.r16_set(R16::HL, result as u16);
        self.tick(Cycles::M(1));
    }

    pub(crate) fn add_sp_e8(&mut self) {
        let imm = self.fetch();
        let sp = self.sp_get();
        let sp_lo = sp as u8;
        self.flag_set(Flag::Z, false);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, (sp_lo & 0x0F) + (imm & 0x0F) > 0x0F);
        self.flag_set(Flag::C, sp_lo as u16 + imm as u16 > 0xFF);
        let imm = imm as i8 as i16;
        let result = sp.wrapping_add_signed(imm) as u16;
        self.sp_set(result);
    }

    pub(crate) fn dec_sp(&mut self) {
        let sp = self.sp_get();
        self.sp_set(sp.wrapping_sub(1));
    }

    pub(crate) fn inc_sp(&mut self) {
        let sp = self.sp_get();
        self.sp_set(sp.wrapping_add(1));
    }

    pub(crate) fn ld_sp_n16(&mut self) {
        let fetch = self.fetch16();
        self.sp_set(fetch);
    }

    pub(crate) fn ld_pn16_sp(&mut self) {
        let adr = self.fetch16();
        let sp = self.sp_get();
        self.mem_write(adr, sp as u8);
        self.mem_write(adr.wrapping_add(1), (sp >> 8) as u8);
    }

    pub(crate) fn ld_hl_sp_e8(&mut self) {
        let imm = self.fetch();
        let sp = self.sp_get();
        let cmp = imm as u16;
        self.flag_set(Flag::Z, false);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, (sp & 0x0F) + (cmp & 0x0F) > 0x0F);
        self.flag_set(Flag::C, (sp & 0xFF) + cmp > 0xFF);
        self.r16_set(R16::HL, sp.wrapping_add_signed(imm as i8 as i16));
        self.tick(Cycles::M(1));
    }

    pub(crate) fn ld_sp_hl(&mut self) {
        let hl = self.r16_get(R16::HL);
        self.sp_set(hl);
    }

    pub(crate) fn pop_r16(&mut self, dst: R16) {
        let u16 = self.pop();
        self.r16_set(dst, u16);
    }

    pub(crate) fn push_r16(&mut self, src: R16) {
        let r16 = self.r16_get(src);
        self.push(r16);
    }

    pub(crate) fn ccf(&mut self) {
        let c = self.flag_get(Flag::C);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, !c);
    }

    pub(crate) fn cpl(&mut self) {
        let a = self.r8_get(R8::A);
        self.r8_set(R8::A, !a);
        self.flag_set(Flag::N, true);
        self.flag_set(Flag::H, true);
    }

    pub(crate) fn daa(&mut self) {
        let c = self.flag_get(Flag::C);
        let n = self.flag_get(Flag::N);
        let h = self.flag_get(Flag::H);
        let mut a = self.r8_get(R8::A);
        if !n {
            if c || a > 0x99 {
                a = a.wrapping_add(0x60);
                self.flag_set(Flag::C, true);
            }
            if h || (a & 0x0F) > 0x09 {
                a = a.wrapping_add(0x06);
            }
        } else {
            if c {
                a = a.wrapping_sub(0x60);
                self.flag_set(Flag::C, true);
            }
            if h {
                a = a.wrapping_sub(0x06);
            }
        }
        self.r8_set(R8::A, a);
        self.flag_set(Flag::Z, a == 0);
        self.flag_set(Flag::H, false);
    }

    pub(crate) fn di(&mut self) {
        self.ime_set(false);
    }

    pub(crate) fn ei(&mut self) {
        if !self.ime_get() {
            self.schedule(1, Slot::EI);
        }
    }

    pub(crate) fn halt(&mut self) {
        self.halted_set(true);
        while self.halted_get() {
            self.interrupt_handler();
            self.tick(Cycles::M(1));
        }
    }

    pub(crate) fn nop(&mut self) {}

    pub(crate) fn scf(&mut self) {
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, true);
    }

    pub(crate) fn stop(&mut self) {
        logger::fatal!("Attempted to execute STOP instruction!");
    }

    pub(crate) fn cb(&mut self) {
        let prefixed = Prefixed::from_byte(self.fetch());
        match prefixed {
            Prefixed::RLC(dst) => self.rlc_r8(dst),
            Prefixed::RLC_PHL => self.rlc_phl(),
            Prefixed::RRC(dst) => self.rrc_r8(dst),
            Prefixed::RRC_PHL => self.rrc_phl(),
            Prefixed::RL(dst) => self.rl_r8(dst),
            Prefixed::RL_PHL => self.rl_phl(),
            Prefixed::RR(dst) => self.rr_r8(dst),
            Prefixed::RR_PHL => self.rr_phl(),
            Prefixed::SLA(dst) => self.sla_r8(dst),
            Prefixed::SLA_PHL => self.sla_phl(),
            Prefixed::SRA(dst) => self.sra_r8(dst),
            Prefixed::SRA_PHL => self.sra_phl(),
            Prefixed::SWAP(dst) => self.swap_r8(dst),
            Prefixed::SWAP_PHL => self.swap_phl(),
            Prefixed::SRL(dst) => self.srl_r8(dst),
            Prefixed::SRL_PHL => self.srl_phl(),
            Prefixed::BIT(bit, src) => self.bit_r8(bit, src),
            Prefixed::BIT_PHL(bit) => self.bit_phl(bit),
            Prefixed::RES(bit, dst) => self.res_r8(bit, dst),
            Prefixed::RES_PHL(bit) => self.res_phl(bit),
            Prefixed::SET(bit, dst) => self.set_r8(bit, dst),
            Prefixed::SET_PHL(bit) => self.set_phl(bit),
        }
    }
}
