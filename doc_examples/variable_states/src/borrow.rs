#![allow(unused)]

pub fn f1() {
    let x = 42;
    let rx = &x;
    let y = x + 1;
    println!("{}", rx);
}

pub fn f2() {
    let mut x = 42;
    let rx = &x;
    let y = x + 1;
    //x = y;
    //error[E0506]: cannot assign to `x` because it is borrowed
    println!("{}", rx);
    // rx out of scope
    x = y;
}

pub fn f3() {
    let mut x = 42;
    let rx = &mut x;
    //x = 43;
    //error[E0506]: cannot assign to `x` because it is borrowed
    //let y = x + 1;
    //error[E0503]: cannot use `x` because it was mutably borrowed
    println!("{}", rx);
    // rx out of scope
    x = 43;
    let y = x + 1;
}