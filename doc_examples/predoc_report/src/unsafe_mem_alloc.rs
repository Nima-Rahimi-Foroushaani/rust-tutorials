use std::alloc::{alloc, dealloc, Layout};
use std::process::abort;

fn caller() {
    let layout = Layout::new::<u8>();
    unsafe {
        let p = alloc_u8();
        *p = 42;
        if *p != 42 {
            dealloc(p, layout);
            dealloc(p, layout);
        }
        dealloc(p, layout);
    }
}

unsafe fn alloc_u8() -> *mut u8
// Function Specification
//@ requires true;
//@ ensures result[..1] |-> _ &*& malloc_block(result, 1);
{
    unsafe {
        let p = alloc(Layout::new::<u8>());
        if p.is_null() {
            abort();
        }
        p
    }
}
