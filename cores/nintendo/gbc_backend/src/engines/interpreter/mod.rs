use super::Engine;
use crate::{
    core::{bus, cpu},
    Core,
};

pub struct Builder {
    pub rom: Vec<u8>,
    pub bootrom: [u8; 256],
}

pub struct Interpreter {
    cpu: cpu::CPU,
    bus: bus::Bus,
}

impl Engine for Interpreter {}

impl Core<Interpreter> {
    pub fn new(builder: Builder) -> Self {
        Self {
            engine: Interpreter {
                cpu: cpu::CPU::new(),
                bus: bus::Bus::new(builder.bootrom, builder.rom),
            },
        }
    }

    pub fn step(&mut self) {}
}
