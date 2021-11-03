#[allow(unused)]
pub fn simple_i32_shared_fail(/*Imm*/ x: & /*Imm*/ i32) -> i32
//@ requires true;
//@ ensures true;
{
    let mut ptr: *const i32 = x;
    {
        let /*Imm*/ n: i32 = 43;
        /* some complex code using "ptr" and "n". really complex! believe me! */
        ptr = & /*Imm*/ n;
    }
    unsafe { *ptr }
}

#[allow(unused)]
pub fn simple_i32_shared_pass(/*Imm*/ x: & /*Imm*/ i32, /*Imm*/ y: & /*Imm*/ i32) -> i32
//@ requires true;
//@ ensures true;
{
    let mut ptr: *const i32 = x;
    {
        /* some complex code using "ptr" and "n". really complex! believe me! */
        let /*Imm*/ n: i32 = 43;
        ptr = y;
    }
    unsafe {
        //@ realize y;
        *ptr
    }
}

#[allow(unused)]
pub fn simple_i32_shared_mut() {
    let local = 42;
    let x = &local;
    println!("x is {}", x);
    let px = x as *const i32 as *mut i32;
    unsafe {
        *px = 43; // seg fault
    }
    println!("x is {}", x);
    // RustBelt infer about &T differently depending on T itself. but here we can see
    // you can Do this mutations based regardless of the type.
}

//Rust Horn COR representation
#[allow(unused)]
pub fn simple_int_pass() -> i32 {
    let mut x: i32 = 42;
    //let *x = 42;

    let mut p: *mut i32 = &mut x;
    //let p = raw x;
    {
        let mut n: i32 = 43;
        //let *n = 43;
        //drop n;
    }
    unsafe {
        *p
        //let *res = copy *p;
    }
    //drop x;
    //return res;
}

#[allow(unused)]
pub fn simple_int_fail() -> i32 {
    let mut x: i32 = 42;
    //let *x = 42;

    let mut p: *mut i32 = &mut x;
    //let p = raw x;
    {
        let mut n: i32 = 43;
        //let *n = 43;
        p = &mut n;
        //let *op = p;
        //let *opp = raw n;
        //swap(*op, *opp);
        //drop opp;
        //let p = *op;

        //drop n;
    }
    unsafe {
        *p
        //let *res = copy *p;
    }
    //drop x;
    //return res;
}
