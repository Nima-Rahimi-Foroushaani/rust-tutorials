fn caller() {
    let layout = std::alloc::Layout::new::<u8>();
    unsafe {
        let p = alloc_u8();
        *p = 42;
        if *p != 42 {
            std::alloc::dealloc(p, layout);
            std::alloc::dealloc(p, layout);
        }
        std::alloc::dealloc(p, layout);
    }
}

unsafe fn alloc_u8()
// Function Specification
//@ requires true;
/*@ ensures integers__(result, 1, false, 1, _) &*& malloc_block(result, 1) &*&
object_pointer_within_limits(result, 1) == true; @*/
-> *mut u8
{
    unsafe {
        let layout = std::alloc::Layout::new::<u8>();
        let p = std::alloc::alloc(layout);
        if p.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        p
    }
}
