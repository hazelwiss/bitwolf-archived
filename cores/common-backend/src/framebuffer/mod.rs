pub mod textures;

use std::{
    marker::PhantomData,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

pub fn fb_3b<T: Canvas + 'static>() -> (AccessR<T>, AccessW<T>) {
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

pub trait Canvas: Sized {
    const WIDTH: usize;
    const HEIGHT: usize;

    fn empty() -> Self;

    fn pitch(&self) -> usize;
}

pub struct Reader<'a, C: Canvas> {
    buf: *mut dyn Buffer<C>,
    _p: PhantomData<&'a ()>,
}

impl<'a, C: Canvas> Reader<'a, C> {
    pub fn read(&self) -> &'a C {
        unsafe { (*self.buf).read() }
    }
}

impl<'a, C: Canvas> Drop for Reader<'a, C> {
    fn drop(&mut self) {
        unsafe { (*self.buf).reader_drop() }
    }
}

pub struct Writer<'a, C: Canvas> {
    buf: *mut dyn Buffer<C>,
    _p: PhantomData<&'a ()>,
}

impl<'a, C: Canvas> Writer<'a, C> {
    pub fn write(&self) -> &'a mut C {
        unsafe { (*self.buf).write() }
    }
}

impl<'a, C: Canvas> Drop for Writer<'a, C> {
    fn drop(&mut self) {
        unsafe { (*self.buf).writer_drop() }
    }
}

trait Buffer<C: Canvas> {
    fn read(&mut self) -> &C;

    fn reader_drop(&mut self);

    fn write(&mut self) -> &mut C;

    fn writer_drop(&mut self);
}

pub struct TrippleBuffering<C: Canvas> {
    buf: [C; 3],
    reader_index: AtomicUsize,
    writer_index: AtomicUsize,
    interm_index: AtomicUsize,
}

impl<C: Canvas> TrippleBuffering<C> {
    fn new() -> Self {
        Self {
            buf: [Canvas::empty(), Canvas::empty(), Canvas::empty()],
            reader_index: AtomicUsize::new(0),
            writer_index: AtomicUsize::new(1),
            interm_index: AtomicUsize::new(2),
        }
    }
}

impl<C: Canvas> Buffer<C> for TrippleBuffering<C> {
    fn read(&mut self) -> &C {
        &self.buf[self.reader_index.load(Ordering::Relaxed)]
    }

    fn reader_drop(&mut self) {
        self.interm_index.store(
            self.reader_index
                .swap(self.interm_index.load(Ordering::SeqCst), Ordering::SeqCst),
            Ordering::SeqCst,
        );
    }

    fn write(&mut self) -> &mut C {
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

pub struct AccessW<C: Canvas> {
    buffer_ptr: Arc<dyn Buffer<C>>,
}

impl<C: Canvas> AccessW<C> {
    pub fn get(&self) -> Writer<C> {
        Writer {
            buf: self.buffer_ptr.as_ref() as *const _ as *mut _,
            _p: PhantomData::default(),
        }
    }
}

unsafe impl<C: Canvas> Send for AccessW<C> {}

pub struct AccessR<C: Canvas> {
    buffer_ptr: Arc<dyn Buffer<C>>,
}

impl<C: Canvas> AccessR<C> {
    pub fn get(&self) -> Reader<C> {
        Reader {
            buf: self.buffer_ptr.as_ref() as *const _ as *mut _,
            _p: PhantomData::default(),
        }
    }
}

unsafe impl<C: Canvas> Send for AccessR<C> {}
