use crossbeam::channel::{Receiver, Sender};

pub struct Demsgq<R: Send, S: Send> {
    receiver: Receiver<R>,
    sender: Sender<S>,
}

impl<R: Send, S: Send> Demsgq<R, S> {
    pub fn send_blocking(&self, s: S) {
        self.sender.send(s);
    }

    /// Returns a bool on whether the message was
    /// sent or not. A value of 'true' for sent
    /// and a value of 'false' if it failed to
    /// send.
    pub fn send(&self, s: S) -> bool {
        self.sender.try_send(s).is_ok()
    }

    pub fn recv_blocking(&self) -> R {
        self.receiver
            .recv()
            .expect("Unable to receive message from message queue")
    }

    pub fn recv(&self) -> Option<R> {
        self.receiver.try_recv().ok()
    }
}

pub fn make_pair<L: Send, R: Send>(capacity: usize) -> (Demsgq<L, R>, Demsgq<R, L>) {
    let (s0, r0) = crossbeam::channel::bounded(capacity);
    let (s1, r1) = crossbeam::channel::bounded(capacity);
    (
        Demsgq {
            receiver: r0,
            sender: s1,
        },
        Demsgq {
            receiver: r1,
            sender: s0,
        },
    )
}
