pub struct BTree<TS: Ord + Copy, E: Ord + Copy> {
    key_to_event: std::collections::BTreeMap<TS, E>,
    event_to_key: std::collections::BTreeMap<E, TS>,
}

impl<TS: Ord + Copy, E: Ord + Copy> BTree<TS, E> {
    pub fn new() -> Self {
        Self {
            key_to_event: std::collections::BTreeMap::new(),
            event_to_key: std::collections::BTreeMap::new(),
        }
    }

    pub fn schedule(&mut self, ts: TS, e: E) {
        if let Some(old_ts) = self.event_to_key.remove(&e) {
            self.event_to_key.insert(e, ts);
            self.key_to_event.remove(&old_ts);
            self.key_to_event.insert(ts, e);
        } else {
            self.event_to_key.insert(e, ts);
            self.key_to_event.insert(ts, e);
        }
    }

    pub fn dispatch(&mut self, ts: TS) -> Option<E> {
        if let Some((&k, _)) = self.key_to_event.first_key_value() {
            if ts >= k {
                let event = self.key_to_event.pop_first().unwrap().1;
                self.event_to_key.remove(&event);
                Some(event)
            } else {
                None
            }
        } else {
            None
        }
    }
}
