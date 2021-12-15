use std::rc::Rc;

fn main() {
    let p1 = Rc::new(42);
    let p2 = p1.clone();
}
