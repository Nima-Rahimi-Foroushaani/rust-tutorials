use std::sync::mpsc;

pub fn sender(tx: mpsc::Sender<String>) {
    let hello: String = String::from("Hello PhD!");
    tx.send(hello).unwrap();
//    println!("{}", hello);
//    error[E0382]: borrow of moved value: `hello`
}

pub fn receiver(rx: mpsc::Receiver<String>) {
    let s = rx.recv().unwrap();
    println!("{}",s);
//    s will be dropped here
}