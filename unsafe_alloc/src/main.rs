fn main()
/*@ requires true;


@*/
// Just a happy comment
//@ ensures true;
{
    //@ open somthing();
    unsafe {
        let layout = std::alloc::Layout::new::<u8>();
        // let _p = std::alloc::alloc(layout) as *mut u8;
    }
}

// fn function1(i: u8)
// //@ requires true;
// //@ ensures true;
// {
//     let j: u8 = 42;
// }
