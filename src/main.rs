use std::{thread, time::Duration};

use channels::channel;

fn main() {
    // A simple demo...

    let (tx, mut rx) = channel();

    let mut tx1 = tx.clone();
    let t1 = thread::spawn(move || {
        tx1.send(("Thread 1", 1));
        thread::sleep(Duration::from_secs(3));
        tx1.send(("Thread 1", 2));
    });

    let mut tx2 = tx.clone();
    let t2 = thread::spawn(move || {
        tx2.send(("Thread 2", 1));
        tx2.send(("Thread 2", 2));
    });

    let t3 = thread::spawn(move || {
        println!("Should be (Thread 1, 1): {:?}", rx.recv());
        println!("Should be (Thread 2, 1): {:?}", rx.recv());
        println!("Should be (Thread 2, 2): {:?}", rx.recv());
        println!("There should be a pause after this message");
        println!("Should be (Thread 1, 2): {:?}", rx.recv());
        println!("Should be None: {:?}", rx.recv());
    });

    for t in [t1, t2, t3] {
        t.join().unwrap();
    }
}
