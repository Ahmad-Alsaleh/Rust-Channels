use std::sync::Arc;

use crate::shared::Shared;

pub struct Sender<T> {
    pub(crate) shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub(crate) fn new(shared: Arc<Shared<T>>) -> Self {
        Self { shared }
    }

    pub fn send(&mut self, value: T) {
        self.shared.inner.lock().unwrap().queue.push_back(value);
        self.shared.available.notify_one();
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.inner.lock().unwrap().n_senders += 1;
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut lock = self.shared.inner.lock().unwrap();
        lock.n_senders -= 1;
        let dropped_last = lock.n_senders == 0;
        drop(lock);
        if dropped_last {
            self.shared.available.notify_one();
        }
    }
}
