use crate::Interpreter;
use core::cpu::{
    instrutions::decode::Bit,
    registers::{Flag, R8},
};

impl Interpreter {
    #[inline(always)]
    pub(crate) fn alu_adc(&mut self, src: u8, carry: bool) {
        let a = self.r8_get(R8::A) as u16;
        let src = src as u16;
        let result = a.wrapping_add(src).wrapping_add(carry as u16);
        self.flag_set(Flag::Z, result as u8 == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, (a & 0x0F) + (src & 0x0F) + (carry as u16) > 0x0F);
        self.flag_set(Flag::C, result > 0xFF);
        self.r8_set(R8::A, result as u8);
    }

    #[inline(always)]
    pub(crate) fn alu_sbc(&mut self, src: u8, carry: bool) {
        let a = self.r8_get(R8::A);
        let result = a.wrapping_sub(src).wrapping_sub(carry as u8);
        self.flag_set(Flag::Z, result as u8 == 0);
        self.flag_set(Flag::N, true);
        self.flag_set(Flag::H, (a & 0x0F) < (src & 0x0F) + carry as u8);
        self.flag_set(
            Flag::C,
            (a as u16) < (src as u16).wrapping_add(carry as u16),
        );
        self.r8_set(R8::A, result);
    }

    #[inline(always)]
    pub(crate) fn alu_cp(&mut self, src: u8) {
        let a = self.r8_get(R8::A);
        let result = a.wrapping_sub(src);
        self.flag_set(Flag::Z, result as u8 == 0);
        self.flag_set(Flag::N, true);
        self.flag_set(Flag::H, a & 0x0F < src & 0x0F);
        self.flag_set(Flag::C, a < src as u8);
    }

    #[inline(always)]
    pub(crate) fn alu_and(&mut self, src: u8) {
        let result = self.r8_get(R8::A) & src;
        self.flag_set(Flag::Z, result == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, true);
        self.flag_set(Flag::C, false);
        self.r8_set(R8::A, result);
    }

    #[inline(always)]
    pub(crate) fn alu_or(&mut self, src: u8) {
        let result = self.r8_get(R8::A) | src;
        self.flag_set(Flag::Z, result == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, false);
        self.r8_set(R8::A, result);
    }

    #[inline(always)]
    pub(crate) fn alu_xor(&mut self, src: u8) {
        let result = self.r8_get(R8::A) ^ src;
        self.flag_set(Flag::Z, result == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, false);
        self.r8_set(R8::A, result);
    }

    #[inline(always)]
    pub(crate) fn generic_bit(&mut self, bit: Bit, src: u8) {
        self.flag_set(Flag::Z, src & bit as u8 == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, true);
    }

    #[inline(always)]
    pub(crate) fn generic_res(&mut self, bit: Bit, dst: u8) -> u8 {
        dst & !(bit as u8)
    }

    #[inline(always)]
    pub(crate) fn generic_set(&mut self, bit: Bit, dst: u8) -> u8 {
        dst | bit as u8
    }

    #[inline(always)]
    pub(crate) fn generic_swap(&mut self, dst: u8) -> u8 {
        self.flag_set(Flag::Z, dst == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, false);
        (dst << 4) | (dst >> 4)
    }

    #[inline(always)]
    pub(crate) fn generic_rl(&mut self, dst: u8) -> u8 {
        let c = self.flag_get(Flag::C);
        let result = (dst << 1) | c as u8;
        self.flag_set(Flag::Z, result == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, dst & Bit::B7 as u8 != 0);
        result
    }

    #[inline(always)]
    pub(crate) fn generic_rlc(&mut self, dst: u8) -> u8 {
        let result = (dst << 1) | (dst >> 7);
        self.flag_set(Flag::Z, result == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, dst & Bit::B7 as u8 != 0);
        result
    }

    #[inline(always)]
    pub(crate) fn generic_rr(&mut self, dst: u8) -> u8 {
        let c = self.flag_get(Flag::C);
        let result = (dst >> 1) | ((c as u8) << 7);
        self.flag_set(Flag::Z, result == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, dst & 1 != 0);
        result
    }

    #[inline(always)]
    pub(crate) fn generic_rrc(&mut self, dst: u8) -> u8 {
        let result = (dst >> 1) | (dst << 7);
        self.flag_set(Flag::Z, result == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, dst & 1 != 0);
        result
    }

    #[inline(always)]
    pub(crate) fn generic_sla(&mut self, dst: u8) -> u8 {
        let result = dst << 1;
        self.flag_set(Flag::Z, result == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, dst & Bit::B7 as u8 != 0);
        result
    }

    #[inline(always)]
    pub(crate) fn generic_sra(&mut self, dst: u8) -> u8 {
        let result = (dst >> 1) | (dst & Bit::B7 as u8);
        self.flag_set(Flag::Z, result == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, dst & 1 != 0);
        result
    }

    #[inline(always)]
    pub(crate) fn generic_srl(&mut self, dst: u8) -> u8 {
        let result = dst >> 1;
        self.flag_set(Flag::Z, result == 0);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, false);
        self.flag_set(Flag::C, dst & 1 != 0);
        result
    }

    #[inline(always)]
    pub(crate) fn generic_dec(&mut self, src: u8) -> u8 {
        let result = src.wrapping_sub(1);
        self.flag_set(Flag::N, true);
        self.flag_set(Flag::H, (src & 0x0F) == 0);
        self.flag_set(Flag::Z, result == 0);
        result
    }

    #[inline(always)]
    pub(crate) fn generic_inc(&mut self, src: u8) -> u8 {
        let result = src.wrapping_add(1);
        self.flag_set(Flag::N, false);
        self.flag_set(Flag::H, (src & 0x0F) == 0x0F);
        self.flag_set(Flag::Z, result == 0);
        result
    }
}
