use crate::{receiver::Receiver, sender::Sender, shared::Shared};
use std::sync::Arc;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared::default());
    let tx = Sender::new(shared.clone());
    let rx = Receiver::new(shared);
    (tx, rx)
}
