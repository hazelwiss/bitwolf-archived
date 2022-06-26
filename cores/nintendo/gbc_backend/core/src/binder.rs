use crate::{
    cpu::{
        instrutions::decode::CC,
        interrupt::InterruptBit,
        registers::{Flag, R16, R8},
    },
    cycles::Cycles,
    emu::event_slots::Slot,
    Emu, Engine,
};

impl<E: Engine> Emu<E> {
    #[inline(always)]
    pub(crate) fn r8_get(&self, r: R8) -> u8 {
        self.cpu.regs().read_r8(r)
    }

    #[inline(always)]
    pub(crate) fn r8_set(&mut self, r: R8, v: u8) {
        self.cpu.regs_mut().write_r8(r, v);
    }

    #[inline(always)]
    pub(crate) fn r16_get(&self, r: R16) -> u16 {
        self.cpu.regs().read_r16(r)
    }

    #[inline(always)]
    pub(crate) fn r16_set(&mut self, r: R16, v: u16) {
        self.cpu.regs_mut().write_r16(r, v);
    }

    #[inline(always)]
    pub(crate) fn pc_get(&self) -> u16 {
        self.cpu.regs().pc_read()
    }

    #[inline(always)]
    pub(crate) fn pc_set(&mut self, val: u16) {
        self.cpu.regs_mut().pc_write(val);
    }

    #[inline(always)]
    pub(crate) fn sp_get(&self) -> u16 {
        self.cpu.regs().sp_read()
    }

    #[inline(always)]
    pub(crate) fn sp_set(&mut self, val: u16) {
        self.cpu.regs_mut().sp_write(val);
    }

    #[inline(always)]
    pub(crate) fn flag_get(&self, f: Flag) -> bool {
        self.cpu.regs().flag_get(f)
    }

    #[inline(always)]
    pub(crate) fn flag_set(&mut self, f: Flag, v: bool) {
        self.cpu.regs_mut().flag_set(f, v);
    }

    #[inline(always)]
    pub(crate) fn check_cond(&self, cc: CC) -> bool {
        match cc {
            CC::NZ => !self.flag_get(Flag::Z),
            CC::Z => self.flag_get(Flag::Z),
            CC::NC => !self.flag_get(Flag::C),
            CC::C => self.flag_get(Flag::C),
        }
    }

    #[inline(always)]
    pub(crate) fn mem_read(&mut self, adr: u16) -> u8 {
        self.tick(Cycles::M(1));
        self.bus.read(adr)
    }

    #[inline(always)]
    pub(crate) fn mem_write(&mut self, adr: u16, val: u8) {
        self.tick(Cycles::M(1));
        self.bus.write(adr, val);
    }

    #[inline(always)]
    pub(crate) fn fetch(&mut self) -> u8 {
        let pc = self.pc_get();
        self.pc_set(pc + 1);
        self.mem_read(pc)
    }

    #[inline(always)]
    pub(crate) fn fetch16(&mut self) -> u16 {
        let lo = self.fetch() as u16;
        let hi = self.fetch() as u16;
        (hi << 8) | lo
    }

    #[inline(always)]
    pub(crate) fn push(&mut self, val: u16) {
        let sp = self.sp_get();
        self.sp_set(sp.wrapping_sub(2));
        self.mem_write(sp.wrapping_sub(1), (val >> 8) as u8);
        self.mem_write(sp.wrapping_sub(2), val as u8);
    }

    #[inline(always)]
    pub(crate) fn pop(&mut self) -> u16 {
        let mut val = 0 as u16;
        let sp = self.sp_get();
        self.sp_set(sp.wrapping_add(2));
        val |= self.mem_read(sp) as u16;
        val |= (self.mem_read(sp.wrapping_add(1)) as u16) << 8;
        val
    }

    #[inline(always)]
    pub(crate) fn phl_get(&mut self) -> u8 {
        self.mem_read(self.r16_get(R16::HL))
    }

    #[inline(always)]
    pub(crate) fn phl_set(&mut self, val: u8) {
        self.mem_write(self.r16_get(R16::HL), val)
    }

    #[inline(always)]
    pub(crate) fn tick(&mut self, cycles: Cycles) {
        self.check_events();
        match cycles {
            Cycles::T(t) => self.bus.tick(t as u64),
            Cycles::M(m) => self.bus.tick((m as u64) << 2),
        }
    }

    pub(crate) fn ime_get(&self) -> bool {
        self.cpu.ime_get()
    }

    pub(crate) fn ime_set(&mut self, val: bool) {
        self.cpu.ime_set(val);
    }

    pub(crate) fn interrupt_handler(&mut self) {
        let ie_f = self.bus.ie_get();
        let if_f = self.bus.if_get();
        let bits = ie_f & if_f & 0x1F;
        if bits != 0 {
            self.halted_set(false);
            if self.ime_get() {
                self.tick(crate::cycles::Cycles::M(3));
                self.push(self.cpu.regs().pc_read());
                let (interrupt, vec) = InterruptBit::from_bits(bits);
                self.pc_set(vec as u16);
                self.bus.if_set(if_f & !(interrupt as u8));
                self.ime_set(false);
            }
        }
    }

    #[inline(always)]
    pub(crate) fn check_events(&mut self) {
        if let Some(slot) = self.bus.dispatch_event() {
            self.handle_event(slot)
        }
    }

    pub(crate) fn halted_get(&self) -> bool {
        self.cpu.halted_get()
    }

    pub(crate) fn halted_set(&mut self, val: bool) {
        self.cpu.halted_set(val);
    }

    pub(crate) fn schedule(&mut self, ts: u64, s: Slot) {
        self.bus.schedule_event(ts, s);
    }
}
