pub fn deref_null() {
    let ptr = 0x0usize as *mut i32;
    unsafe {
        *ptr = 42;
    }
}

pub fn breaks_ty_sys(rrx: &mut &mut i32) {
    let ptr = rrx as *mut &mut i32 as *mut *mut i32;
    unsafe {
        *ptr = 0x0usize as *mut i32;
    }
}

pub struct Vector {
    pub ptr: *mut i32,
    pub len: usize,
}

impl Vector {
    pub fn at<'a>(&'a mut self, idx: usize) -> &'a mut i32 {
        unsafe { &mut *self.ptr.add(idx) }
    }
}
