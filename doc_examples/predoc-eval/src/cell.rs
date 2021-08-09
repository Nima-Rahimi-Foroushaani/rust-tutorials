use std::cell::Cell;
use std::thread;

pub fn f1() {
    let c = Cell::new(42);
//  let h = thread::spawn(|| {println!("{}",c.get())});
//  `Cell<i32>` cannot be shared between threads safely
//  help: the trait `Sync` is not implemented for `Cell<i32>`
//  note: required because of the requirements on the impl of `Send` for `&Cell<i32>`
    let h = thread::spawn(move || {println!("{}",c.get())});
    h.join().unwrap();
}

//pub fn set(&self, val: T) {}
//pub fn get(&self) -> T {}