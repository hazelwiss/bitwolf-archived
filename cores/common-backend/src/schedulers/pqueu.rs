use priority_queue::PriorityQueue;
use std::hash::Hash;

#[derive(Debug)]
pub struct PQueue<TS: Ord + Copy, S: Ord + Copy + Hash> {
    pq: priority_queue::PriorityQueue<S, std::cmp::Reverse<TS>>,
}

impl<TS: Ord + Copy, S: Ord + Copy + Hash> PQueue<TS, S> {
    pub fn new() -> Self {
        Self {
            pq: PriorityQueue::new(),
        }
    }
}

impl<TS: Ord + Copy, S: Ord + Copy + Hash> PQueue<TS, S> {
    pub fn schedule(&mut self, ts: TS, s: S) {
        self.deschedule(s);
        self.pq.push(s, std::cmp::Reverse(ts));
    }

    pub fn dispatch(&mut self, ts: TS) -> Option<S> {
        if let Some((_, &p)) = self.pq.peek() {
            if p.0 <= ts {
                Some(self.pq.pop().unwrap().0)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn deschedule(&mut self, s: S) {
        let mut new_priority_queue = PriorityQueue::new();
        self.pq.iter().filter(|c| *c.0 != s).for_each(|e| {
            new_priority_queue.push(*e.0, *e.1);
        });
        self.pq = new_priority_queue;
    }
}
