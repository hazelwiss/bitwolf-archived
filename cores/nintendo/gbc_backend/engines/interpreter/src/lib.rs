#![feature(mixed_integer_ops)]

mod events;
mod instructions;
mod interrupts;

use common_backend::schedulers;

pub struct Builder {
    pub rom: Vec<u8>,
    pub bootrom: [u8; 256],
}

pub struct Interpreter {
    cpu: cpu::CPU,
    bus: bus::Bus,
    scheduler: schedulers::BTree<u64, events::Event>,
    cycle_counter: u64,
}

impl Interpreter {
    pub fn new(builder: Builder) -> Self {
        Self {
            cpu: cpu::CPU::new(),
            bus: bus::Bus::new(builder.bootrom, builder.rom),
            scheduler: schedulers::BTree::new(),
            cycle_counter: 0,
        }
    }

    pub fn step(&mut self) {
        self.interrupt_handler();
        self.fetch_decode_execute();
    }
}
