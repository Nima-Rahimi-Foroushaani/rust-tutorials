use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

fn main() {
    let i = 42;
    let (snd, rcv): (Sender<&i32>, Receiver<&i32>) = mpsc::channel();
    snd.send(&i);
}
