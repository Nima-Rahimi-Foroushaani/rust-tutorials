fn f() {
    let v = 100; let rv = &v;
    let p = &rv as *const &i32 as *mut &i32 as *mut usize;
    unsafe { *p = 42; /* `rv` is dangeling */ }
}