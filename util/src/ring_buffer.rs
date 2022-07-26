use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub struct RB<T: Copy, const SIZE: usize> {
    pop_index: AtomicUsize,
    push_index: AtomicUsize,
    len: AtomicUsize,
    buffer: [T; SIZE],
}

unsafe impl<T: Copy, const SIZE: usize> Send for RB<T, SIZE> {}

impl<T: Copy, const SIZE: usize> RB<T, SIZE> {
    pub fn new(val: T) -> Self {
        Self {
            pop_index: AtomicUsize::new(0),
            push_index: AtomicUsize::new(0),
            len: AtomicUsize::new(0),
            buffer: [val; SIZE],
        }
    }

    pub fn push(&self, val: T) {
        while self.len.load(Ordering::Acquire) == SIZE {}
        let push_index = self.push_index.load(Ordering::Acquire);
        self.push_index
            .store((push_index + 1) % SIZE, Ordering::Release);
        unsafe {
            let ptr = &self.buffer[push_index] as *const _ as *mut _;
            (*ptr) = val;
        }

        self.len
            .store(self.len.load(Ordering::Acquire) + 1, Ordering::Release)
    }

    pub fn pop(&self) -> Option<T> {
        if self.len.load(Ordering::Acquire) > 0 {
            let pop_index = self.pop_index.load(Ordering::Acquire);
            self.pop_index
                .store((pop_index + 1) % SIZE, Ordering::Release);
            let val = self.buffer[pop_index];
            self.len
                .store(self.len.load(Ordering::Acquire) - 1, Ordering::Release);
            Some(val)
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
