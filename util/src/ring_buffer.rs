use std::sync::{Arc, Mutex};

pub struct RB<T: Copy, const SIZE: usize> {
    pop_index: usize,
    push_index: usize,
    len: usize,
    buffer: [T; SIZE],
}

impl<T: Copy + Default, const SIZE: usize> RB<T, SIZE> {
    pub fn new() -> Self {
        Self {
            pop_index: 0,
            push_index: 0,
            len: 0,
            buffer: [T::default(); SIZE],
        }
    }
}

impl<T: Copy, const SIZE: usize> RB<T, SIZE> {
    pub fn push(&mut self, val: T) {
        if self.len < SIZE {
            self.buffer[self.push_index] = val;
            self.push_index = (self.push_index + 1) % SIZE;
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            let val = self.buffer[self.pop_index];
            self.pop_index = (self.pop_index + 1) % SIZE;
            self.len -= 1;
            Some(val)
        } else {
            None
        }
    }
}

type MPRBInternal<T, const SIZE: usize> = Arc<Mutex<RB<T, SIZE>>>;

/// Mutex protected ring buffer
pub struct MPRB<T: Copy, const SIZE: usize> {
    rb: MPRBInternal<T, SIZE>,
}

impl<T: Copy + Default, const SIZE: usize> MPRB<T, SIZE> {
    pub fn new() -> Self {
        Self {
            rb: Arc::new(Mutex::new(RB::new())),
        }
    }
}

impl<T: Copy, const SIZE: usize> MPRB<T, SIZE> {
    pub fn pusher(&self) -> RBPusher<T, SIZE> {
        RBPusher {
            rb: self.rb.clone(),
        }
    }

    pub fn poper(&self) -> RBPoper<T, SIZE> {
        RBPoper {
            rb: self.rb.clone(),
        }
    }
}

pub struct RBPusher<T: Copy, const SIZE: usize> {
    rb: MPRBInternal<T, SIZE>,
}

impl<T: Copy, const SIZE: usize> RBPusher<T, SIZE> {
    pub fn push(&self, val: T) {
        let mut rb = self.rb.lock().unwrap();
        rb.push(val);
    }
}

pub struct RBPoper<T: Copy, const SIZE: usize> {
    rb: MPRBInternal<T, SIZE>,
}

impl<T: Copy, const SIZE: usize> RBPoper<T, SIZE> {
    pub fn pop(&self) -> Option<T> {
        let mut rb = self.rb.lock().unwrap();
        rb.pop()
    }
}
