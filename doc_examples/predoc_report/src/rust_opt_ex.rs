use std::cell::Cell;

fn do_something(c: &Cell<u32>, u: u32) {
    c.set(u)
}

fn f<'a>(c: &'a Cell<u32>) {
    for i in 0..100 {
        do_something(c, i);
        println!("{:?}", *c)
    }
}
