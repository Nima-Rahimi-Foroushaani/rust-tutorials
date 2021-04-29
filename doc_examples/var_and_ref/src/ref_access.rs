#![allow(unused)]

pub fn cmp() {
    let x: &i32 = &42;
    let y: i32 = 42;
    
    //assert_eq!(x, y);
    // error[E0277]: can't compare `&i32` with `i32`
    assert_eq!(x, &y);
    assert!(x == &y);
    // "==" compares referents of two references
    // see: https://doc.rust-lang.org/std/primitive.reference.html
}

pub fn read() {
    let x: &i32 = &42;
    
    //let y: i32 = x;
    // error[E0308]: mismatched types
    let y: i32 = *x;
}

pub fn assign() {
    let mut x: i32 = 0;
    let y = &mut x;
    //y = 42;
    //error[E0308]: mismatched types
    *y = 42;
}