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

fn raw_own_deref_unsafe() {
    let mut p = 0xFFFFFFFFusize as *const Box<i32>;
    {
        let b = Box::new(42);
        p = &b;
    }
    
    unsafe{
//        let b1 = *p;
    }
}

fn raw_mut_deref_unsafe() {
    let mut x: i32 = 42;
    let mut rx = &mut x;
    let p: *mut i32 = rx;
    unsafe{
        let rx1 = *p;
    }
}

fn raw_immut_deref_unsafe() {
    let p: *const &i32;
    {
        let x = 42;
        let rrx = &&x;
        p = rrx;        
    }
    unsafe {
        // let x1: &i32 = *p;
    }
}

fn raw_move() {
    let mut b = Box::new(42);
    let p = &mut b as *mut Box<i32>;
    unsafe {
        // cannot move something below raw pointer but we can get a mutable reference
        let br = &mut *p;
    }

}
