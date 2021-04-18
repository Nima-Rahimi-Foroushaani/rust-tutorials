#![allow(unused)]
pub fn f1() {
    let x:i32;
    //println!("{}", x);
    //error[E0381]: borrow of possibly-uninitialized variable: `x`
    //let y = x;
    //error[E0381]: use of possibly-uninitialized variable: `x`
    x = 42;
    println!("{}", x);
}

pub fn f2() {
    let  str0 = String::from("Hello");
    let str1 = str0; //move
    //let str2 = str0;
    // error[E0382]: use of moved value: `str0`
    //println!("{}", str0);
    //error[E0382]: borrow of moved value: `str0`
    //str0 = String::from("world");
    // error[E0384]: cannot assign twice to immutable variable `str`
    println!("{}", str1);
}

pub fn f3() {
    let mut  str0 = String::from("Hello");
    let str1 = str0;
    str0 = String::from("world");
    println!("{} {}", str1, str0);
}