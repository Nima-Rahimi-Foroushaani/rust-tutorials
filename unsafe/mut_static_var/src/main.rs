#![allow(unused)]

struct Person {
    height : u8,
    weight : u8,
}

static mut ADAM: Person = Person {
    height : 180,
    weight : 80
};

fn main() {
    println!("Adam is {} cm and {} kg", unsafe{ADAM.height}, unsafe{ADAM.weight});
}
