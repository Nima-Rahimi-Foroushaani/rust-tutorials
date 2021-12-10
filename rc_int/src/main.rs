#![allow(unused)]
/**
 * for this version of calculus without multithreading there is no need for implementing !Sync
 */
mod cell_u32 {
    pub struct CellU32 {
        value: u32,
    }

    impl CellU32 {
        pub fn new(v: u32) -> CellU32 {
            let r = CellU32 { value: v };
            return r;
        }
        pub fn get(self: &Self) -> u32 {
            return self.value;
        }
        pub fn set(self: &Self, value: u32) {
            let ptr = &self.value as *const u32 as *mut u32;
            unsafe {
                *ptr = value;
            }
        }
    }
}

mod rc_u32 {
    use crate::cell_u32::CellU32;
    use std::alloc::{alloc, dealloc, Layout};

    struct RcBoxU32 {
        value: u32,
        strong: CellU32,
    }

    pub struct RcU32 {
        ptr: *mut RcBoxU32,
    }
    impl RcU32 {
        pub fn new(v: u32) -> RcU32 {
            let layout = Layout::new::<RcBoxU32>();
            unsafe {
                let ptr = alloc(layout);
                let ptr1 = ptr as *mut RcBoxU32;
                let strong = CellU32::new(1);
                *ptr1 = RcBoxU32 {
                    value: v,
                    strong: strong,
                };
                let r = RcU32 { ptr: ptr1 };
                return r;
            }
        }
    }
    impl Clone for RcU32 {
        fn clone(self: &Self) -> RcU32 {
            unsafe {
                let count = 1 + (*self.ptr).strong.get();
                (*self.ptr).strong.set(count);
            }

            let r = RcU32 { ptr: self.ptr };
            return r;
        }
    }
    impl Drop for RcU32 {
        fn drop(self: &mut Self) {
            unsafe {
                let count = (*self.ptr).strong.get() - 1;
                match count == 0 {
                    true => {
                        let ptr = self.ptr as *mut u8;
                        let layout = Layout::new::<RcBoxU32>();
                        dealloc(ptr, layout)
                    }
                    false => (),
                }
            }
        }
    }
}

fn main() {}
