//use std::sync::mpsc;

//pub fn sender(tx: mpsc::Sender<&String>) {
//    let hello: String = String::from("Hello PhD!");
//    tx.send(&hello).unwrap();
//}
//
//pub fn receiver(rx: mpsc::Receiver<&String>) {
//    let s = rx.recv().unwrap();
//    println!("{}",s);
//}

pub fn f1() {
    let mut s1 = String::from("Computer ");
    let mut s2 = String::from("Computer ");
    s1.push_str("Science");
    s2.push_str("Wetenschappen");
    println!("{}",s1);
//    Computer Science
    println!("{}",s2);
//    Computer Wetenschappen
    let d = which_is_dutch(&s1, &s2);
//    s1 = String::from("Computer");
//    error[E0506]: cannot assign to `s1` because it is borrowed
    println!("\"{}\" is Dutch",d);
}

pub fn which_is_dutch<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {s1} else {s2}
}