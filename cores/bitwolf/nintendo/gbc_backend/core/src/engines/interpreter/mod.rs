mod instructions;

use crate::{Emu, Engine};

pub struct Interpreter;

impl Engine for Interpreter {
    type EngineData = ();
}

impl Emu<Interpreter> {
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
        self.interrupt_handler();
        self.fetch_decode_execute();
    }
}
