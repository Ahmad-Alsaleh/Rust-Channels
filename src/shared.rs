use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

pub(crate) struct Inner<T> {
    pub(crate) queue: VecDeque<T>,
    pub(crate) n_senders: usize,
}

impl<T> Default for Inner<T> {
    fn default() -> Self {
        Self {
            queue: Default::default(),
            n_senders: 1,
        }
    }
}

pub(crate) struct Shared<T> {
    pub(crate) inner: Mutex<Inner<T>>,
    pub(crate) available: Condvar,
}

impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            available: Default::default(),
        }
    }
}
