#![allow(unused)]

fn main() {
    
//    let mut num = 42;
//    
//    let mptr_num = &mut num as *mut i32;
//    //let mptr_num = &num as *mut i32; // casting immutable to mutable is not allowd
//
//    let cptr1_num = &mut num as *const i32;
//    let cptr2_num = &num as *const i32;
//    // creating raw pointer is not unsafe but trying to dereference them may be.
    
    let ptr : *mut i32 = 0xFFFFFFFFusize as *mut i32;
    
    // two mutable pointers to same address
    let ptr1 = ptr;
    
    unsafe {
        // memory access violation
        *ptr = ptr1 as i32;
    }
    
    //println!("num is {}", num);
}
