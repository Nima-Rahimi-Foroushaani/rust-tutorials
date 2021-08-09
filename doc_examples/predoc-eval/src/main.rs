mod ownership;
mod borrowing;
mod cell;

use std::sync::mpsc;
use std::thread;

fn main() {
    {
        let (tx, rx) = mpsc::channel();
        let txh = thread::spawn(|| ownership::sender(tx));
        let rxh = thread::spawn(|| ownership::receiver(rx));
        txh.join().unwrap();
        rxh.join().unwrap();
    }
    
//    {
//        let (tx, rx) = mpsc::channel();
//        let txh = thread::spawn(|| borrowing::sender(tx));
//        let rxh = thread::spawn(|| borrowing::receiver(rx));
//        txh.join().unwrap();
//        rxh.join().unwrap();
//    }
    borrowing::f1();
    
    cell::f1();
}
