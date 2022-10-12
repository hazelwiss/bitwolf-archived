#![allow(dead_code)]

use crossbeam_channel::{RecvError, SendError, TryRecvError, TrySendError};

pub struct BiMsgQ<S: Sync + Send, R: Sync + Send> {
    sender: crossbeam_channel::Sender<S>,
    receiver: crossbeam_channel::Receiver<R>,
}

impl<S: Sync + Send, R: Sync + Send> Clone for BiMsgQ<S, R> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
        }
    }
}

impl<S: Sync + Send, R: Sync + Send> BiMsgQ<S, R> {
    pub fn new(size: usize) -> (BiMsgQ<S, R>, BiMsgQ<R, S>) {
        let (s0, r0) = crossbeam_channel::bounded(size);
        let (s1, r1) = crossbeam_channel::bounded(size);
        (
            BiMsgQ {
                sender: s0,
                receiver: r1,
            },
            BiMsgQ {
                sender: s1,
                receiver: r0,
            },
        )
    }

    #[inline]
    pub fn send_blocking(&self, send: S) -> Result<(), SendError<S>> {
        self.sender.send(send)
    }

    #[inline]
    pub fn send(&self, send: S) -> Result<bool, TrySendError<S>> {
        match self.sender.try_send(send) {
            Ok(_) => Ok(true),
            Err(err) => match err {
                TrySendError::Full(_) => Ok(false),
                err => Err(err),
            },
        }
    }

    #[inline]
    pub fn recv_blocking(&self) -> Result<R, RecvError> {
        self.receiver.recv()
    }

    #[inline]
    pub fn recv(&self) -> Result<Option<R>, TryRecvError> {
        match self.receiver.try_recv() {
            Ok(v) => Ok(Some(v)),
            Err(err) => match err {
                crossbeam_channel::TryRecvError::Empty => Ok(None),
                err => Err(err),
            },
        }
    }
}
