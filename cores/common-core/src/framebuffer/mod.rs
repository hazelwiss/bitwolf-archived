pub mod textures;

use std::{
    marker::PhantomData,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use textures::TextureInfo;

pub fn fb_3b<T: TextureInfo + 'static>() -> (AccessR<T>, AccessW<T>) {
    let triple = TrippleBuffering::<T>::new();
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

pub struct Reader<'a, T: TextureInfo> {
    buf: *mut dyn Buffer<T>,
    _p: PhantomData<&'a ()>,
}

impl<'a, T: TextureInfo> Reader<'a, T> {
    pub fn read(&self) -> &'a T {
        unsafe { (*self.buf).read() }
    }
}

impl<'a, T: TextureInfo> Drop for Reader<'a, T> {
    fn drop(&mut self) {
        unsafe { (*self.buf).reader_drop() }
    }
}

pub struct Writer<'a, T: TextureInfo> {
    buf: *mut dyn Buffer<T>,
    _p: PhantomData<&'a ()>,
}

impl<'a, T: TextureInfo> Writer<'a, T> {
    pub fn write(&self) -> &'a mut T {
        unsafe { (*self.buf).write() }
    }
}

impl<'a, T: TextureInfo> Drop for Writer<'a, T> {
    fn drop(&mut self) {
        unsafe { (*self.buf).writer_drop() }
    }
}

trait Buffer<T: TextureInfo> {
    fn read(&mut self) -> &T;

    fn reader_drop(&mut self);

    fn write(&mut self) -> &mut T;

    fn writer_drop(&mut self);
}

pub struct TrippleBuffering<T: TextureInfo> {
    buf: [T; 3],
    reader_index: AtomicUsize,
    writer_index: AtomicUsize,
    interm_index: AtomicUsize,
}

impl<T: TextureInfo> TrippleBuffering<T> {
    fn new() -> Self {
        Self {
            buf: [T::default(), T::default(), T::default()],
            reader_index: AtomicUsize::new(0),
            writer_index: AtomicUsize::new(1),
            interm_index: AtomicUsize::new(2),
        }
    }
}

impl<T: TextureInfo> Buffer<T> for TrippleBuffering<T> {
    fn read(&mut self) -> &T {
        &self.buf[self.reader_index.load(Ordering::Relaxed)]
    }

    fn reader_drop(&mut self) {
        self.interm_index.store(
            self.reader_index
                .swap(self.interm_index.load(Ordering::SeqCst), Ordering::SeqCst),
            Ordering::SeqCst,
        );
    }

    fn write(&mut self) -> &mut T {
        &mut self.buf[self.writer_index.load(Ordering::Relaxed)]
    }

    fn writer_drop(&mut self) {
        self.interm_index.store(
            self.writer_index
                .swap(self.interm_index.load(Ordering::SeqCst), Ordering::SeqCst),
            Ordering::SeqCst,
        );
    }
}

pub struct AccessW<T: TextureInfo> {
    buffer_ptr: Arc<dyn Buffer<T>>,
}

impl<T: TextureInfo> AccessW<T> {
    pub fn get(&self) -> Writer<T> {
        Writer {
            buf: self.buffer_ptr.as_ref() as *const _ as *mut _,
            _p: PhantomData::default(),
        }
    }
}

unsafe impl<T: TextureInfo> Send for AccessW<T> {}

pub struct AccessR<T: TextureInfo> {
    buffer_ptr: Arc<dyn Buffer<T>>,
}

impl<T: TextureInfo> AccessR<T> {
    pub fn get(&self) -> Reader<T> {
        Reader {
            buf: self.buffer_ptr.as_ref() as *const _ as *mut _,
            _p: PhantomData::default(),
        }
    }
}

unsafe impl<T: TextureInfo> Send for AccessR<T> {}
