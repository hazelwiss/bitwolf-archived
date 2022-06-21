#![feature(mixed_integer_ops)]

pub(crate) mod internal;

pub struct Builder {
    pub rom: Vec<u8>,
    pub bootrom: [u8; 256],
}

pub struct Interpreter {
    cpu: cpu::CPU,
    bus: bus::Bus,
}

impl Interpreter {
    pub fn new(builder: Builder) -> Self {
        Self {
            cpu: cpu::CPU::new(),
            bus: bus::Bus::new(builder.bootrom, builder.rom),
        }
    }
}
