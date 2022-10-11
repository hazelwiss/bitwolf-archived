pub mod arm7;
pub mod arm9;

use alloc::{boxed::Box, string::String, vec::Vec};
use core::marker::PhantomData;

use crate::core::Core;

#[derive(Debug)]
pub struct Instr {
    string_repr: String,
    byte_repr: Vec<u8>,
}

impl Instr {
    pub fn string_repr(&self) -> &str {
        &self.string_repr
    }

    pub fn byte_repr(&self) -> &[u8] {
        &self.byte_repr
    }

    pub fn byte_len(&self) -> usize {
        self.byte_repr.len()
    }
}

#[derive(Debug)]
pub enum Disasm {
    Instr(Instr),
}

impl Disasm {
    pub fn byte_len(&self) -> usize {
        match self {
            Disasm::Instr(instr) => instr.byte_len(),
        }
    }
}

pub struct DisasmEntry<T> {
    pub disasm: T,
    pub address: u64,
}

pub struct Iter<'a, T> {
    cur_adr: u64,
    entries: *mut Disasm,
    cur_entry: usize,
    count: usize,
    _p: PhantomData<&'a T>,
}

impl<'a, T> Iter<'a, T> {
    pub fn from_raw(cur_adr: u64, entries: *mut Disasm, count: usize) -> Self {
        Self {
            cur_adr,
            entries,
            count,
            cur_entry: 0,
            _p: Default::default(),
        }
    }
}

macro_rules! next_impl {
    ($ptr:ident, $index:ident, { $($e:tt)* }, $($tt:tt)*) => {
        impl<'a> Iterator for Iter<'a, DisasmEntry<$($tt)*>> {
            type Item = DisasmEntry<$($tt)*>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count > 0{
                    let $ptr = self.entries;
                    let $index = self.cur_adr;
                    let cur_entry = unsafe { $($e)* };
                    let cur_adr = self.cur_adr;
                    self.cur_adr += cur_entry.byte_len() as u64;
                    self.cur_entry += 1;
                    self.count -= 1;
                    Some(DisasmEntry {
                        disasm: cur_entry,
                        address: cur_adr,
                    })
                } else{
                    None
                }
            }
        }
    };
}

next_impl! {entries, index, { entries.add(index as usize).read() }, Disasm}
next_impl! {entries, index, { &*entries.add(index as usize) }, &'a Disasm}
next_impl! {entries, index, { &mut *entries.add(index as usize) }, &'a mut Disasm}

#[derive(Debug, Default)]
pub struct Disassembly {
    pub disasm: Box<[Disasm]>,
    pub start_address: u64,
}

impl<'a> Disassembly {
    pub fn iter(&self) -> Iter<'a, DisasmEntry<&'a Disasm>> {
        Iter::from_raw(
            self.start_address,
            self.disasm.as_ptr() as *mut _,
            self.disasm.len(),
        )
    }

    pub fn iter_mut(&mut self) -> Iter<'a, DisasmEntry<&'a mut Disasm>> {
        Iter::from_raw(
            self.start_address,
            self.disasm.as_mut_ptr(),
            self.disasm.len(),
        )
    }
}

impl IntoIterator for Disassembly {
    type Item = DisasmEntry<Disasm>;
    type IntoIter = Iter<'static, Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        Iter::from_raw(
            self.start_address,
            self.disasm.as_mut_ptr(),
            self.disasm.len(),
        )
    }
}

impl<'a> IntoIterator for &'a Disassembly {
    type Item = DisasmEntry<&'a Disasm>;
    type IntoIter = Iter<'a, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Disassembly {
    type Item = DisasmEntry<&'a mut Disasm>;
    type IntoIter = Iter<'a, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

trait Disassemble {
    fn disassemble(core: &Core, start: u32, len: u32) -> Disassembly;
}

pub fn disassemble_arm9(core: &Core, start: u32, len: u32) -> Disassembly {
    arm9::ARM9Disasm::disassemble(core, start, len)
}

pub fn disassemble_arm7() -> Disassembly {
    todo!()
}
