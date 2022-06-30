use super::Buffer;
use common_core::textures::TextureInfo;
use std::{marker::PhantomData, sync::Arc};

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

pub struct AccessW<T: TextureInfo> {
    pub(super) buffer_ptr: Arc<dyn Buffer<T>>,
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
    pub(super) buffer_ptr: Arc<dyn Buffer<T>>,
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
