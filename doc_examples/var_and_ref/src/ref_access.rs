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
    
    let ptr_42: *const i32 = x;
    let ptr_y: *const i32 = &y;
    assert_ne!(ptr_42, ptr_y);
}

pub fn read() {
    // Active_Context:
    // Symbolic_Store:
    // Symbolic_Mem_Stage:
    // Assumptions:
    let x: &i32 = &42;
    // Active_Context: x: source(i32)
    // Symbolic_Store: x: _x
    // Symbolic_Mem_Stage: (_x, 42)
    // Assumptions:
    
    //let y: i32 = x;
    // error[E0308]: mismatched types
    
    let y: i32 = *x;
    // Active_Context: x: source(i32), y: i32
    // Symbolic_Store: x: _x, y: 42
    // Symbolic_Mem_Stage: (_x, 42)
    // Assumptions:
}

pub fn assign() {
    let mut x: i32 = 0;
    let y = &mut x;
    //y = 42;
    //error[E0308]: mismatched types
    *y = 42;
}


// Below function has copied from Rust standard library documentation
// https://doc.rust-lang.org/std/primitive.reference.html
pub fn aliasing() {
    use std::ptr;
    
    let five = 5;
    let other_five = 5;
    let five_ref = &five;
    let same_five_ref = &five;
    let other_five_ref = &other_five;
    
    assert!(five_ref == same_five_ref);
    assert!(five_ref == other_five_ref);
    
    assert!(ptr::eq(five_ref, same_five_ref));
    
    assert!(!ptr::eq(five_ref, other_five_ref));
}