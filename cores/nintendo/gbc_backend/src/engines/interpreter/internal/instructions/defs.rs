use crate::engines::interpreter::Interpreter;
use cpu::{
    cycles::Cycles,
    instrutions::decode::RSTVec,
    instrutions::decode::CC,
    registers::{Flag, R16, R8},
};

/*
    Helper function.
*/
impl Interpreter {
    #[inline(always)]
    fn r8_get(&self, r: R8) -> u8 {
        match r {
            R8::A => todo!(),
            R8::B => todo!(),
            R8::C => todo!(),
            R8::D => todo!(),
            R8::E => todo!(),
            R8::H => todo!(),
            R8::L => todo!(),
        }
    }

    #[inline(always)]
    fn r8_set(&mut self, r: R8, v: u8) {
        match r {
            R8::A => todo!(),
            R8::B => todo!(),
            R8::C => todo!(),
            R8::D => todo!(),
            R8::E => todo!(),
            R8::H => todo!(),
            R8::L => todo!(),
        }
    }

    #[inline(always)]
    fn r16_get(&self, r: R16) -> u16 {
        match r {
            R16::AF => todo!(),
            R16::BC => todo!(),
            R16::DE => todo!(),
            R16::HL => todo!(),
        }
    }

    #[inline(always)]
    fn r16_set(&mut self, r: R16, v: u16) {
        match r {
            R16::AF => todo!(),
            R16::BC => todo!(),
            R16::DE => todo!(),
            R16::HL => todo!(),
        }
    }

    #[inline(always)]
    fn pc_get(&self) -> u16 {
        self.cpu.regs().read_pc()
    }

    #[inline(always)]
    fn pc_set(&mut self, val: u16) {
        self.cpu.regs_mut().write_pc(val);
    }

    #[inline(always)]
    fn sp_get(&self) -> u16 {
        self.cpu.regs().read_sp()
    }

    #[inline(always)]
    fn sp_set(&mut self, val: u16) {
        self.cpu.regs_mut().write_sp(val);
    }

    #[inline(always)]
    fn flag_get(&self, f: Flag) -> bool {
        match f {
            Flag::Z => todo!(),
            Flag::N => todo!(),
            Flag::H => todo!(),
            Flag::C => todo!(),
        }
    }

    #[inline(always)]
    fn flag_set(&mut self, f: Flag, v: bool) {
        match f {
            Flag::Z => todo!(),
            Flag::N => todo!(),
            Flag::H => todo!(),
            Flag::C => todo!(),
        }
    }

    fn check_cond(&self, cc: CC) -> bool {
        false
    }

    #[inline(always)]
    fn mem_read(&mut self, adr: u16) -> u8 {
        0
    }

    #[inline(always)]
    fn mem_write(&mut self, adr: u16, val: u8) {}

    #[inline(always)]
    fn fetch(&mut self) -> u8 {
        0
    }

    #[inline(always)]
    fn fetch16(&mut self) -> u16 {
        let lo = self.fetch() as u16;
        let hi = self.fetch() as u16;
        (hi << 8) | lo
    }

    #[inline(always)]
    fn push(&mut self, val: u16) {}

    #[inline(always)]
    fn pop(&mut self) -> u16 {
        0
    }

    #[inline(always)]
    fn phl_get(&mut self) -> u8 {
        self.mem_read(self.r16_get(R16::HL))
    }

    #[inline(always)]
    fn phl_set(&mut self, val: u8) {
        self.mem_write(self.r16_get(R16::HL), val)
    }

    #[inline(always)]
    fn tick(&mut self, cycles: Cycles) {}
}

/*
    Generic funtions.
*/
impl Interpreter {
    #[inline(always)]
    fn alu_generic(&mut self, src: u8, carry: bool) -> u8 {
        let a = self.r8_get(R8::A) as u16;
        let src = src as u16;
        let result = a.wrapping_add(src).wrapping_add(carry as u16);
        self.flag_set(Flag::H, (a & 0x0F) + (src & 0x0F) > 0x0F);
        self.flag_set(Flag::C, result > 0xFF);
        self.flag_set(Flag::Z, result as u8 == 0);
        result as u8
    }

    #[warn(unused_results)]
    fn alu_add(&mut self, src: u8, carry: bool) {
        self.flag_set(Flag::N, false);
        let val = self.alu_generic(src, carry);
        self.r8_set(R8::A, val);
    }

    #[warn(unused_results)]
    fn alu_sub(&mut self, src: u8, carry: bool) {
        self.flag_set(Flag::N, true);
        let val = self.alu_generic((!src).wrapping_add(1), carry);
        self.r8_set(R8::A, val);
    }

    fn alu_cp(&mut self, src: u8) {
        self.flag_set(Flag::N, true);
        let val = self.alu_generic((!src).wrapping_add(1), false);
    }

    fn alu_and(&mut self, src: u8) {
        let result = self.r8_get(R8::A) & src;
        self.flag_set(Flag::Z, result == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, true);
        self.flag_set(Flag::C, false);
        self.r8_set(R8::A, result);
    }

    fn alu_or(&mut self, src: u8) {
        let result = self.r8_get(R8::A) | src;
        self.flag_set(Flag::Z, result == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, false);
        self.r8_set(R8::A, result);
    }

    fn alu_xor(&mut self, src: u8) {
        let result = self.r8_get(R8::A) ^ src;
        self.flag_set(Flag::Z, result == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, false);
        self.r8_set(R8::A, result);
    }

    fn generic_bit(&mut self, src: u8) {}

    fn generic_res(&mut self, dst: u8) -> u8 {
        0
    }

    fn generic_set(&mut self, dst: u8) -> u8 {
        0
    }

    fn generic_swap(&mut self, dst: u8) -> u8 {
        0
    }

    fn generic_rl(&mut self, dst: u8) -> u8 {
        0
    }

    fn generic_rlc(&mut self, dst: u8) -> u8 {
        0
    }

    fn generic_rr(&mut self, dst: u8) -> u8 {
        0
    }

    fn generic_rrc(&mut self, dst: u8) -> u8 {
        0
    }

    fn generic_sla(&mut self, dst: u8) -> u8 {
        0
    }

    fn generic_sra(&mut self, dst: u8) -> u8 {
        0
    }

    fn generic_srl(&mut self, dst: u8) -> u8 {
        0
    }

    fn generic_dec(&mut self, src: u8) -> u8 {
        let result = src.wrapping_sub(1);
        self.flag_set(Flag::N, true);
        self.flag_set(Flag::H, (result & 0x0F) != 0);
        self.flag_set(Flag::Z, result == 0);
        result
    }

    fn generic_inc(&mut self, src: u8) -> u8 {
        let result = src.wrapping_add(1);
        self.flag_set(Flag::N, true);
        self.flag_set(Flag::H, (result & 0x0F) == 0x0F);
        self.flag_set(Flag::Z, result == 0);
        result
    }
}

impl Interpreter {
    pub fn adc_r8(&mut self, src: R8) {
        self.alu_add(self.r8_get(src), self.flag_get(Flag::C));
    }

    pub fn adc_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_add(phl, self.flag_get(Flag::C));
    }

    pub fn adc_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_add(fetch, self.flag_get(Flag::C));
    }

    pub fn add_r8(&mut self, src: R8) {
        self.alu_add(self.r8_get(src), false);
    }

    pub fn add_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_add(phl, false);
    }

    pub fn add_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_add(fetch, false);
    }

    pub fn and_r8(&mut self, src: R8) {
        self.alu_and(self.r8_get(src));
    }

    pub fn and_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_and(phl);
    }

    pub fn and_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_and(fetch);
    }

    pub fn cp_r8(&mut self, src: R8) {
        self.alu_cp(self.r8_get(src));
    }

    pub fn cp_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_cp(phl);
    }

    pub fn cp_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_cp(fetch);
    }

    pub fn dec_r8(&mut self, dst: R8) {
        let result = self.generic_dec(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub fn dec_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_dec(phl);
        self.phl_set(result);
    }

    pub fn inc_r8(&mut self, dst: R8) {
        let result = self.generic_inc(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub fn inc_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_inc(phl);
        self.phl_set(result);
    }

    pub fn or_r8(&mut self, src: R8) {
        self.alu_or(self.r8_get(src));
    }

    pub fn or_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_or(phl);
    }

    pub fn or_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_or(fetch);
    }

    pub fn sbc_r8(&mut self, src: R8) {
        self.alu_sub(self.r8_get(src), self.flag_get(Flag::C));
    }

    pub fn sbc_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_sub(phl, self.flag_get(Flag::C));
    }

    pub fn sbc_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_sub(fetch, self.flag_get(Flag::C));
    }

    pub fn sub_r8(&mut self, src: R8) {
        self.alu_sub(self.r8_get(src), false);
    }

    pub fn sub_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_sub(phl, false);
    }

    pub fn sub_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_sub(fetch, false);
    }

    pub fn xor_r8(&mut self, src: R8) {
        self.alu_xor(self.r8_get(src));
    }

    pub fn xor_phl(&mut self) {
        let phl = self.phl_get();
        self.alu_xor(phl);
    }

    pub fn xor_n8(&mut self) {
        let fetch = self.fetch();
        self.alu_xor(fetch);
    }

    pub fn add_hl_r16(&mut self, src: R16) {
        let hl = self.r16_get(R16::HL) as u32;
        let src = self.r16_get(src) as u32;
        let result = hl + src;
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, (hl & 0x0FFF) + (src & 0x0FFF) > 0x0FFF);
        self.flag_set(Flag::C, result > 0xFFFF);
        self.r16_set(R16::HL, result as u16);
        self.tick(Cycles::M(1));
    }

    pub fn dec_r16(&mut self, dst: R16) {
        self.r16_set(dst, self.r16_get(dst).wrapping_sub(1));
    }

    pub fn inc_r16(&mut self, dst: R16) {
        self.r16_set(dst, self.r16_get(dst).wrapping_add(1));
    }

    pub fn bit_r8(&mut self, src: R8) {
        self.generic_bit(self.r8_get(src));
    }

    pub fn bit_phl(&mut self) {
        let phl = self.phl_get();
        self.generic_bit(phl);
    }

    pub fn res_r8(&mut self, dst: R8) {
        let result = self.generic_res(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub fn res_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_res(phl);
        self.phl_set(result);
    }

    pub fn set_r8(&mut self, dst: R8) {
        let result = self.generic_set(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub fn set_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_set(phl);
        self.phl_set(result);
    }

    pub fn swap_r8(&mut self, dst: R8) {
        let result = self.generic_swap(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub fn swap_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_swap(phl);
        self.phl_set(result);
    }

    pub fn rl_r8(&mut self, dst: R8) {
        let result = self.generic_rl(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub fn rl_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_rl(phl);
        self.phl_set(result);
    }

    pub fn rla(&mut self) {
        let result = self.generic_rl(self.r8_get(R8::A));
        self.r8_set(R8::A, result);
        self.flag_set(Flag::Z, false);
    }

    pub fn rlc_r8(&mut self, dst: R8) {
        let result = self.generic_rlc(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub fn rlc_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_rlc(phl);
        self.phl_set(result);
    }

    pub fn rlca(&mut self) {
        let result = self.generic_rlc(self.r8_get(R8::A));
        self.r8_set(R8::A, result);
        self.flag_set(Flag::Z, false);
    }

    pub fn rr_r8(&mut self, dst: R8) {
        let result = self.generic_rr(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub fn rr_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_rr(phl);
        self.phl_set(result);
    }

    pub fn rra(&mut self) {
        let result = self.generic_rr(self.r8_get(R8::A));
        self.r8_set(R8::A, result);
        self.flag_set(Flag::Z, false);
    }

    pub fn rrc_r8(&mut self, dst: R8) {
        let result = self.generic_rrc(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub fn rrc_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_rrc(phl);
        self.phl_set(result);
    }

    pub fn rrca(&mut self) {
        let result = self.generic_rrc(self.r8_get(R8::A));
        self.r8_set(R8::A, result);
        self.flag_set(Flag::Z, false);
    }

    pub fn sla_r8(&mut self, dst: R8) {
        let result = self.generic_sla(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub fn sla_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_sla(phl);
        self.phl_set(result);
    }

    pub fn sra_r8(&mut self, dst: R8) {
        let result = self.generic_sra(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub fn sra_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_sra(phl);
        self.phl_set(result);
    }

    pub fn srl_r8(&mut self, dst: R8) {
        let result = self.generic_srl(self.r8_get(dst));
        self.r8_set(dst, result);
    }

    pub fn srl_phl(&mut self) {
        let phl = self.phl_get();
        let result = self.generic_srl(phl);
        self.phl_set(result);
    }

    pub fn ld_r8_r8(&mut self, dst: R8, src: R8) {
        self.r8_set(dst, self.r8_get(src));
    }

    pub fn ld_r8_n8(&mut self, dst: R8) {
        let fetch = self.fetch();
        self.r8_set(dst, fetch);
    }

    pub fn ld_r16_n16(&mut self, dst: R16) {
        let fetch = self.fetch16();
        self.r16_set(dst, fetch);
    }

    pub fn ld_phl_r8(&mut self, src: R8) {
        self.phl_set(self.r8_get(src));
    }

    pub fn ld_phl_n8(&mut self) {
        let fetch = self.fetch();
        self.phl_set(fetch);
    }

    pub fn ld_r8_phl(&mut self, dst: R8) {
        let phl = self.phl_get();
        self.r8_set(dst, phl);
    }

    pub fn ld_pr16_a(&mut self, dst: R16) {
        let ptr = self.r16_get(dst);
        self.mem_write(ptr, self.r8_get(R8::A));
    }

    pub fn ld_pn16_a(&mut self) {
        let ptr = self.fetch16();
        self.mem_write(ptr, self.r8_get(R8::A));
    }

    pub fn ldh_pc_a(&mut self) {
        let ptr = 0xFF00 | self.r8_get(R8::C) as u16;
        self.mem_write(ptr, self.r8_get(R8::A));
    }

    pub fn ld_a_pr16(&mut self, src: R16) {
        let read = self.mem_read(self.r16_get(src));
        self.r8_set(R8::A, read);
    }

    pub fn ld_a_pn16(&mut self) {
        let fetch = self.fetch16();
        let read = self.mem_read(fetch);
        self.r8_set(R8::A, read);
    }

    pub fn ldh_a_pn8(&mut self) {
        let adr = 0xFF00 | self.fetch() as u16;
        let read = self.mem_read(adr);
        self.r8_set(R8::A, read);
    }

    pub fn ldh_a_pc(&mut self) {
        let adr = 0xFF00 | self.r8_get(R8::C) as u16;
        let read = self.mem_read(adr);
        self.r8_set(R8::A, read);
    }

    pub fn ld_phli_a(&mut self) {
        let hl = self.r16_get(R16::HL);
        self.r16_set(R16::HL, hl.wrapping_add(1));
        self.mem_write(hl, self.r8_get(R8::A));
    }

    pub fn ld_phld_a(&mut self) {
        let hl = self.r16_get(R16::HL);
        self.r16_set(R16::HL, hl.wrapping_sub(1));
        self.mem_write(hl, self.r8_get(R8::A));
    }

    pub fn ld_a_hli(&mut self) {
        let hl = self.r16_get(R16::HL);
        self.r16_set(R16::HL, hl.wrapping_add(1));
        let read = self.mem_read(hl);
        self.r8_set(R8::A, read);
    }

    pub fn ld_a_hld(&mut self) {
        let hl = self.r16_get(R16::HL);
        self.r16_set(R16::HL, hl.wrapping_sub(1));
        let read = self.mem_read(hl);
        self.r8_set(R8::A, read);
    }

    pub fn call(&mut self) {
        self.push(self.pc_get());
        let adr = self.fetch16();
        self.pc_set(adr);
        self.tick(Cycles::M(1));
    }

    pub fn call_cc(&mut self, cc: CC) {
        let adr = self.fetch16();
        if self.check_cond(cc) {
            self.push(self.pc_get());
            self.pc_set(adr);
            self.tick(Cycles::M(1));
        }
    }

    pub fn jp_hl(&mut self) {
        self.pc_set(self.r16_get(R16::HL));
    }

    pub fn jp(&mut self) {
        let adr = self.fetch16();
        self.pc_set(adr);
        self.tick(Cycles::M(1));
    }

    pub fn jp_cc(&mut self, cc: CC) {
        let adr = self.fetch16();
        if self.check_cond(cc) {
            self.pc_set(adr);
            self.tick(Cycles::M(1));
        }
    }

    pub fn jr(&mut self) {
        let imm = self.fetch() as i8 as i16;
        let pc = self.pc_get();
        self.pc_set(pc.wrapping_add_signed(imm) as u16);
        self.tick(Cycles::M(1));
    }

    pub fn jr_cc(&mut self, cc: CC) {
        let imm = self.fetch() as i8 as i16;
        if self.check_cond(cc) {
            let pc = self.pc_get();
            self.pc_set(pc.wrapping_add_signed(imm) as u16);
            self.tick(Cycles::M(1));
        }
    }

    pub fn ret_cc(&mut self, cc: CC) {
        if self.check_cond(cc) {
            self.ret()
        }
        self.tick(Cycles::M(1));
    }

    pub fn ret(&mut self) {
        let adr = self.pop();
        self.pc_set(adr);
        self.tick(Cycles::M(1));
    }

    pub fn reti(&mut self) {
        self.ret();
        self.ei();
    }

    pub fn rst(&mut self, vec: RSTVec) {
        self.push(self.pc_get());
        self.pc_set(vec as u16);
        self.tick(Cycles::M(1));
    }

    pub fn add_hl_sp(&mut self) {
        let hl = self.r16_get(R16::HL) as u32;
        let sp = self.sp_get() as u32;
        let result = hl + sp;
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, (hl & 0x0FFF) + (sp & 0x0FFF) < 0x0FFF);
        self.flag_set(Flag::C, result > 0xFFFF);
        self.r16_set(R16::HL, result as u16);
        self.tick(Cycles::M(1));
    }

    pub fn add_sp_e8(&mut self) {
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

    pub fn dec_sp(&mut self) {
        let sp = self.sp_get();
        self.sp_set(sp.wrapping_sub(1));
    }

    pub fn inc_sp(&mut self) {
        let sp = self.sp_get();
        self.sp_set(sp.wrapping_add(1));
    }

    pub fn ld_sp_n16(&mut self) {
        let fetch = self.fetch16();
        self.sp_set(fetch);
    }

    pub fn ld_pn16_sp(&mut self) {
        let adr = self.fetch16();
        let sp = self.sp_get();
        self.mem_write(adr, sp as u8);
        self.mem_write(adr.wrapping_add(1), (sp >> 8) as u8);
    }

    pub fn ld_hl_sp_e8(&mut self) {
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

    pub fn ld_sp_hl(&mut self) {
        self.sp_set(self.r16_get(R16::HL));
    }

    pub fn pop_af(&mut self) {
        todo!();
    }

    pub fn pop_r16(&mut self, dst: R16) {
        let val = self.pop();
        self.r16_set(dst, val);
    }

    pub fn push_r16(&mut self, src: R16) {
        self.push(self.r16_get(src));
    }

    pub fn ccf(&mut self) {
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, !self.flag_get(Flag::C))
    }

    pub fn cpl(&mut self) {
        self.r8_set(R8::A, !self.r8_get(R8::A));
        self.flag_set(Flag::N, true);
        self.flag_set(Flag::H, true);
    }

    pub fn daa(&mut self) {
        todo!()
    }

    pub fn di(&mut self) {
        todo!()
    }

    pub fn ei(&mut self) {
        todo!()
    }

    pub fn halt(&mut self) {
        todo!()
    }

    pub fn nop(&mut self) {}

    pub fn scf(&mut self) {
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, true);
    }

    pub fn stop(&mut self) {
        logger::fatal!("Attempted to execute STOP instruction!");
    }
}
