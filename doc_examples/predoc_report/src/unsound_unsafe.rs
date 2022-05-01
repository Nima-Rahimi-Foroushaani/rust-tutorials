pub fn stupid() {
    let ptr = 0x000000usize as *mut i32;
    unsafe {
        *ptr = 42;
    }
}
// Even worse
pub struct Vector {
    pub ptr: *mut i32,
    pub len: usize,
}

impl Vector {
    pub fn at<'a>(&'a mut self, idx: usize) -> &'a mut i32 {
        unsafe { &mut *self.ptr.add(idx) }
    }
}
