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

//Rust COR representation
#[allow(unused)]
pub fn simple_int_pass() {
    let mut result:i32 = 0;
    //let *result = 0;
    let mut x:i32 = 42;
    //let *x = 42;

    let mut p:*mut i32 = &mut x;
    //intro alpha;
    //let mx = mutbor_alpha x;
    //let px = raw mx;
    //let *p = px;
    //drop mx;
    //now alpha;
    {
        let mut y:i32 = 43;
        //let *y = 43;
        //drop y;
    }
    unsafe {
        //unsafe;
        result = *p;
        //let px = *p;
        //let *cx = copy *px;
        //swap(*result,*cx);
        //drop cx;
        //let *p = px;
        //safe;
    }
    //drop p;
    //drop x;
    //drop result;
}

#[allow(unused)]
pub fn simple_int_fail() {
    let mut result:i32 = 0;
    //let *result = 0;
    let mut x:i32 = 42;
    //let *x = 42;

    let mut p:*mut i32 = &mut x;
    //intro alpha;
    //let mx = mutbor_alpha x;
    //let px = raw mx;
    //let *p = px;
    //drop mx;
    //now alpha;
    {
        let mut y:i32 = 43;
        //let *y = 43;
        p = &mut y;
        //intro beta;
        //let my = mutbor_beta y;
        //let py = raw my;
        //let *p1 = py;
        //drop my;
        //now beta;
        //swap(*p,*p1);
        //drop p1;
        
        //drop y;
    }
    unsafe {
        //unsafe;
        result = *p;
        //let px = *p;
        //let *cx = copy *px;
        //swap(*result,*cx);
        //drop cx;
        //let *p = px;
        //safe;
    }
    //drop p;
    //drop x;
    //drop result;
}
