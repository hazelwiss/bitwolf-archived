use std::{
    cell::UnsafeCell,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

pub struct RB<T: Copy, const SIZE: usize> {
    pop_index: AtomicUsize,
    push_index: AtomicUsize,
    buffer: UnsafeCell<[T; SIZE]>,
}

unsafe impl<T: Copy, const SIZE: usize> Send for RB<T, SIZE> {}
unsafe impl<T: Copy, const SIZE: usize> Sync for RB<T, SIZE> {}

impl<T: Copy, const SIZE: usize> RB<T, SIZE> {
    pub fn new(val: T) -> Self {
        Self {
            pop_index: AtomicUsize::new(0),
            push_index: AtomicUsize::new(0),
            buffer: UnsafeCell::new([val; SIZE]),
        }
    }

    pub fn push(&self, val: T) {
        let push_index = self.push_index.load(Ordering::Relaxed);
        let new_push_index = (push_index + 1) % SIZE;
        let _ = self
            .pop_index
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |pop_index| {
                if pop_index == new_push_index {
                    Some(new_push_index + 1)
                } else {
                    None
                }
            });
        unsafe {
            self.buffer.get().cast::<T>().add(push_index).write(val);
        }
        self.push_index.store(new_push_index, Ordering::Release);
    }

    pub fn pop(&self) -> Option<T> {
        if let Ok(pop_index) =
            self.pop_index
                .fetch_update(Ordering::Acquire, Ordering::Relaxed, |pop_index| {
                    let new_pop_index = (pop_index + 1) % SIZE;
                    if new_pop_index == self.push_index.load(Ordering::Relaxed) {
                        None
                    } else {
                        Some(new_pop_index)
                    }
                })
        {
            Some(unsafe { self.buffer.get().cast::<T>().add(pop_index).read() })
        } else {
            None
        }
    }
}

/// Mutex protected ring buffer
pub fn spawn<T: Copy + Default, const SIZE: usize>() -> (RBPusher<T, SIZE>, RBPoper<T, SIZE>) {
    spawn_filled(T::default())
}

pub fn spawn_filled<T: Copy, const SIZE: usize>(val: T) -> (RBPusher<T, SIZE>, RBPoper<T, SIZE>) {
    let rb = Arc::new(RB::new(val));
    (RBPusher { rb: rb.clone() }, RBPoper { rb })
}

pub struct RBPusher<T: Copy, const SIZE: usize> {
    rb: Arc<RB<T, SIZE>>,
}

impl<T: Copy, const SIZE: usize> RBPusher<T, SIZE> {
    pub fn push(&self, val: T) {
        self.rb.push(val);
    }
}

pub struct RBPoper<T: Copy, const SIZE: usize> {
    rb: Arc<RB<T, SIZE>>,
}

impl<T: Copy, const SIZE: usize> RBPoper<T, SIZE> {
    pub fn pop(&self) -> Option<T> {
        self.rb.pop()
    }
}
