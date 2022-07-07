use super::super::{
    access::{AccessR, AccessW},
    Buffer, TextureInfo,
};
use std::sync::{Arc, Mutex};

struct Indexes {
    reader_index: usize,
    writer_index: usize,
    interm_index: usize,
    swap_ready: bool,
}

pub struct TripleBuffer<T: TextureInfo> {
    buf: Box<[T; 3]>,
    indexes: Mutex<Indexes>,
}

impl<T: TextureInfo> TripleBuffer<T> {
    fn new() -> Self {
        Self {
            buf: Box::new([T::default(), T::default(), T::default()]),
            indexes: Mutex::new(Indexes {
                reader_index: 0,
                writer_index: 0,
                interm_index: 0,
                swap_ready: false,
            }),
        }
    }
}

impl<T: TextureInfo> Buffer<T> for TripleBuffer<T> {
    fn read(&mut self) -> &T {
        let index = self.indexes.lock().unwrap().reader_index;
        &self.buf[index]
    }

    fn reader_drop(&mut self) {
        let mut indexes = self.indexes.lock().unwrap();
        if indexes.swap_ready {
            let tmp = indexes.interm_index;
            indexes.interm_index = indexes.reader_index;
            indexes.reader_index = tmp;
            indexes.swap_ready = false;
        }
    }

    fn write(&mut self) -> &mut T {
        let index = self.indexes.lock().unwrap().writer_index;
        &mut self.buf[index]
    }

    fn writer_drop(&mut self) {
        let mut indexes = self.indexes.lock().unwrap();
        let tmp = indexes.interm_index;
        indexes.interm_index = indexes.writer_index;
        indexes.writer_index = tmp;
        indexes.swap_ready = true;
    }
}

pub fn new<T: TextureInfo + 'static>() -> (AccessR<T>, AccessW<T>) {
    let triple = TripleBuffer::<T>::new();
    let arc = Arc::new(triple);
    (
        AccessR {
            buffer_ptr: arc.clone(),
        },
        AccessW {
            buffer_ptr: arc.clone(),
        },
    )
}
