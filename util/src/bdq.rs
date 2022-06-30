use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

pub struct Bdq<R, S> {
    receiver: Receiver<R>,
    sender: SyncSender<S>,
}

impl<R, S> Bdq<R, S> {
    pub fn try_recv(&self) -> Option<R> {
        self.receiver.try_recv().ok()
    }

    pub fn recv(&self) -> R {
        self.receiver.recv().unwrap()
    }

    pub fn try_send(&self, msg: S) -> bool {
        self.sender.try_send(msg).is_ok()
    }

    pub fn send(&self, msg: S) {
        self.sender.send(msg).unwrap();
    }
}

pub fn new_pair<M0, M1>(size: usize) -> (Bdq<M0, M1>, Bdq<M1, M0>) {
    let (s0, r0) = sync_channel(size);
    let (s1, r1) = sync_channel(size);
    (
        Bdq::<M0, M1> {
            receiver: r0,
            sender: s1,
        },
        Bdq::<M1, M0> {
            receiver: r1,
            sender: s0,
        },
    )
}
