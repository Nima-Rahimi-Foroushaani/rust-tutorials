#![allow(unused)]

pub fn unsafe_deref_fn_fail (x: & /*Imm*/ () ) -> () /* () is the unit type */
//@ requires true
//@ ensures true
{
    let /*Imm*/ ptr: *const () = x;
    unsafe {
        *ptr
    }
}

pub fn unsafe_deref_fn_pass (x: & /*Imm*/ () ) -> () /* () is the unit type */
//@ requires true
//@ ensures true
{
    let /*Imm*/ ptr: *const () = x;
    unsafe {
        /* @realize x; */
        *ptr
    }
}