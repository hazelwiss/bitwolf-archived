use std::{
    path::PathBuf,
    sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError},
};

pub struct FileReader<T: Send> {
    receiver: Receiver<(T, PathBuf)>,
    sender: SyncSender<(T, PathBuf)>,
}

impl<T: Send + 'static> FileReader<T> {
    pub fn new(queue_size: usize) -> Self {
        let (sender, receiver) = sync_channel(queue_size);
        Self { receiver, sender }
    }

    pub fn read_file(&self, msg_type: T, filters: Vec<(&'static str, &'static [&'static str])>) {
        let sender = self.sender.clone();
        std::thread::spawn(move || {
            let mut file_reader = rfd::FileDialog::new();
            for (filter_name, filter_ext) in filters {
                file_reader = file_reader.add_filter(filter_name, filter_ext);
            }
            let file = file_reader.pick_file();
            if let Some(file) = file {
                if let Err(err) = sender.send((msg_type, file)) {
                    logger::warning!(
                        "Unabe lto send message to message queue with warning '{err:?}'"
                    );
                }
            } else {
                logger::warning!("Unable to open file in file dialogue.")
            }
        });
    }

    pub fn retrieve_respons(&self) -> Option<(T, PathBuf)> {
        match self.receiver.try_recv() {
            Ok((msg_type, file)) => Some((msg_type, file)),
            Err(TryRecvError::Empty) => None,
            Err(err) => {
                logger::warning!("Attempted to retrieve from message queue with warning '{err:?}'");
                None
            }
        }
    }
}
