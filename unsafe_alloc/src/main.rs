fn main()
//@ requires true;
//@ ensures true;
{
    unsafe {
        let layout = std::alloc::Layout::new::<i8>();
        let _p = std::alloc::alloc(layout) as *mut i8;
    }
}
