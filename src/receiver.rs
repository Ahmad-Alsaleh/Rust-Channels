use std::{collections::VecDeque, sync::Arc};

use crate::shared::Shared;

pub struct Receiver<T> {
    pub(crate) shared: Arc<Shared<T>>,
    pub(crate) buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    pub(crate) fn new(shared: Arc<Shared<T>>) -> Self {
        Self {
            shared,
            buffer: VecDeque::new(),
        }
    }

    pub fn recv(&mut self) -> Option<T> {
        if let Some(value) = self.buffer.pop_front() {
            return Some(value);
        }

        let mut lock = self.shared.inner.lock().unwrap();
        loop {
            match lock.queue.pop_front() {
                Some(value) => {
                    std::mem::swap(&mut lock.queue, &mut self.buffer);
                    return Some(value);
                }
                None if lock.n_senders == 0 => return None,
                None => {
                    lock = self.shared.available.wait(lock).unwrap();
                }
            };
        }
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}
