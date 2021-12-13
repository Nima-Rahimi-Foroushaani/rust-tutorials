#![allow(unused)]
/**
 * for this version of calculus without multithreading there is no need for implementing !Sync
 */
mod cell_u32 {
    pub struct CellU32 {
        value: u32,
    }
    // pub struct cell_u32::CellU32{nat}

    impl CellU32 {
        pub fn new(v: u32) -> CellU32 {
            let r = CellU32 { value: v };
            return r;
        }
        /***
         * pub safe fn cell_u32::new<|>(v:own nat) -> own cell_u32::CellU32 {
         * let *r = cell_u32::CellU32{*x};
         * return r;
         * }
         */

        pub fn get(self: &Self) -> u32 {
            return self.value;
        }
        /***
         * pub safe fn cell_u32::get<alpha|>(self: immut'alpha cell_u32::CellU32) -> own nat {
         * let value = self.0;
         * let *r = copy *value;
         * drop value;
         * return r;
         * }
         */
        pub fn set(self: &Self, value: u32) {
            let ptr = &self.value as *const u32 as *mut u32;
            unsafe {
                *ptr = value;
            }
        }
        /***
         * pub safe fn cell_u32::set<alpha|>(self: immut'alpha cell_u32::CellU32, value: own nat) -> own unit {
         * let cv = self.0;
         * let ptr = raw cv;
         * unsafe;
         * swap(*ptr, *value);
         * safe;
         * drop ptr;
         * drop cv;
         * drop value;
         * let *r = unit{};
         * return r;
         * }
         */
    }
}

mod rc_u32 {
    use crate::cell_u32;
    use crate::cell_u32::CellU32;
    use std::alloc::{alloc, dealloc, Layout};

    struct RcBoxU32 {
        value: u32,
        strong: CellU32,
    }
    // private struct rc_u32::RcBoxU32{nat, cell_u32::CellU32}

    pub struct RcU32 {
        ptr: *mut RcBoxU32,
    }
    // pub struct rc_u32::RcU32{raw rc_u32::RcBoxU32}
    impl RcU32 {
        pub fn new(value: u32) -> RcU32 {
            let layout = Layout::new::<RcBoxU32>();
            unsafe {
                let ptr = alloc(layout);
                let ptr1 = ptr as *mut RcBoxU32;
                let strong = CellU32::new(1);
                *ptr1 = RcBoxU32 {
                    value: value,
                    strong: strong,
                };
                let r = RcU32 { ptr: ptr1 };
                return r;
            }
        }
        /***
         * pub safe fn rc_u32::new<|>(value: own nat) -> own rc_u32::RcU32 {
         * unsafe;
         * let *strong = 1;
         * let *sc = cell_u32::new(strong);
         * let *rcbox = rc_u32::RcBoxU32{*value, *sc};
         * let *ptr = alloc rc_u32::RcBoxU32;
         * swap(*ptr, *rcbox);
         * let *r = rc_u32::RcU32{*ptr};
         * drop rcbox;
         * safe;
         * return r;
         * }
         */
    }
    impl Clone for RcU32 {
        /*pub*/
        fn clone(self: &Self) -> RcU32 {
            let ptr = self.ptr;
            let strong;
            unsafe {
                // let count = 1 + (*self.ptr).strong.get();
                let imm_rcb = &*ptr;
                strong = &(imm_rcb.strong);
            }
            let count = cell_u32::CellU32::get(strong);
            let count1 = count + 1;
            // unsafe { (*self.ptr).strong.set(count1) };
            cell_u32::CellU32::set(strong, count1);

            let r = RcU32 { ptr: ptr };
            return r;
        }
        /***
         * pub safe fn rc_u32::clone<alpha|>(self: immut'alpha rc_u32::RcU32) -> own rc_u32::RcU32 {
         * let imm_ptr_rcb = self.0;
         * let *o_ptr_rcb = copy *imm_ptr_rcb;
         * let ptr_rcb = *imm_ptr_rcb;
         * let ptr_strong = ptr_rcb.1;
         * unsafe;
         * intro beta;
         * let imm_strong = immut'beta ptr_strong;
         * safe;
         * let *o_imm_strong = imm_strong;
         * let *o_imm_strong1 = copy *o_imm_strong;
         * let imm_strong = *o_imm_strong;
         * let imm_strong1 = *o_imm_strong1;
         * let *count = cell_u32::get<beta>(imm_strong1);
         * let *one = 1;
         * let count1 = *count + *one;
         * let *dummy = cell_u32::set<beta>(imm_strong, count1);
         * now beta;
         * drop dummy;
         * let *r = rc_u32::RcU32{*o_ptr_rcb};
         * return r;
         * }
         */
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
        /***
         * private safe rc_u32::drop<alpha|>(self: mut'alpha rc_u32::RcU32) -> own unit {
         * let mut_ptr_rcbox = self.0;
         * let ptr_rcbox = *mut_ptr_rcbox;
         * let *o_ptr_rcbox = ptr_rcbox;
         * let *o_ptr_rcbox1 = copy *ptr_rcbox;
         * let ptr_rcbox = *o_ptr_rcbox;
         * let ptr_rcbox1 = o_ptr_rcbox1;
         * let ptr_strong = ptr_rcbox1.1;
         * unsafe;
         * intro beta;
         * let imm_strong = immut'beta ptr_strong;
         * safe;
         * let *count = cell_u32::get(imm_strong);
         * now beta;
         * let *one = 1;
         * let *is_last = *count == *one;
         * match *is_last:bool
         * *y0 => goto L_FALSE;
         * *y1 => goto L_TRUE;
         * L_False:
         * drop y0:goto L_FIN;
         * L_TRUE:
         * dealloc ptr_rcbox;
         * drop y1:goto L_FIN;
         * L_FIN:
         * drop one;
         * drop count;
         * drop ptr_strong;
         * let *r = ();
         * return r;
         * }
         */
    }
}

fn main() {}
