
mod instructions;

use std::ops::{Deref, DerefMut};

pub struct Builder {
    pub rom: Vec<u8>,
    pub bootrom: [u8; 256],
}

pub struct Interpreter {
    emu: core::Emu,
}

impl Deref for Interpreter {
    type Target = core::Emu;

    fn deref(&self) -> &Self::Target {
        &self.emu
    }
}

impl DerefMut for Interpreter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.emu
    }
}

impl Interpreter {
    pub fn new(builder: Builder) -> Self {
        Self {
            emu: core::Emu::new(builder.bootrom, builder.rom),
        }
    }

    pub fn step(&mut self) {
        //let pc = self.cpu.regs().read_pc();
        //let sp = self.cpu.regs().read_sp();
        //let hl = self.cpu.regs().read_r16(cpu::registers::R16::HL);
        //let bc = self.cpu.regs().read_r16(cpu::registers::R16::BC);
        //let de = self.cpu.regs().read_r16(cpu::registers::R16::DE);
        //let a = self.cpu.regs().read_r8(cpu::registers::R8::A);
        //let z = self.cpu.regs().get_flag(cpu::registers::Flag::Z);
        //let n = self.cpu.regs().get_flag(cpu::registers::Flag::N);
        //let h = self.cpu.regs().get_flag(cpu::registers::Flag::H);
        //let c = self.cpu.regs().get_flag(cpu::registers::Flag::C);
        //println!(
        //    "PC -> {pc:04X} : SP: {sp:04X}, BC: {bc:04X}, DE: {de:04X}, HL: {hl:04X}, A: {a:02X}, Z:{} N:{} H:{} C:{}", z as u8, n as u8, h as u8, c as u8
        //);
        self.emu.interrupt_handler();
        self.fetch_decode_execute();
    }
}
