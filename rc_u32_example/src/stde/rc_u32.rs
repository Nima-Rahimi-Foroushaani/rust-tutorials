use crate::stde;
use std::alloc::{alloc, dealloc, Layout};
// use std::ops::Deref;
#[derive(Debug)]
struct RcBoxU32 {
    value: u32,
    strong: stde::cell_u32::CellNat,
}
// private struct stde::rc_u32::RcBoxU32{nat, stde::cell_u32::CellU32}

#[derive(Debug)]
pub struct RcU32 {
    ptr: *mut RcBoxU32,
}
// pub struct stde::rc_u32::RcU32{raw stde::rc_u32::RcBoxU32}

pub fn new(v: u32) -> RcU32 {
    let layout = Layout::new::<RcBoxU32>();
    let ptr_rcb;
    let mut_rcb;
    unsafe {
        let ptr = alloc(layout);
        ptr_rcb = ptr as *mut RcBoxU32;
        mut_rcb = &mut *ptr_rcb
    };
    let strong = stde::cell_u32::new(1);
    let rcb_data = RcBoxU32 {
        value: v,
        strong: strong,
    };
    *mut_rcb = dbg!(rcb_data);
    let r = RcU32 { ptr: ptr_rcb };
    return r;
}
/***
 * pub safe fn stde::rc_u32::new<|>(v: own nat) -> own stde::rc_u32::RcU32 {
 * let *sz = sizeof stde::rc_u32::RcBoxU32;
 * unsafe;
 * let ptr_rcb = alloc *sz;
 * ptr_rcb as raw RcBoxU32;
 * intro alpha;
 * let mut_rcb = mut'alpha ptr_rcb;
 * safe;
 *
 * let *one = 1;
 * let strong = stde::cell_u32::new<>(one);
 * let *rcb_data = stde::rc_u32::RcBoxU32{*v, *strong};
 * swap(*mut_rcb, *rcb_data);
 * drop mut_rcb;
 * now alpha;
 * let *o_ptr_rcb = ptr_rcb;
 * let *r = stde::rc_u32::RcU32{*o_ptr_rcb};
 * drop rcb_data;
 * drop sz;
 * return r;
 * }
 */

pub fn clone<'a>(this: &'a RcU32) -> RcU32 {
    let imm_ptr_rcb = &this.ptr;
    let ptr_rcb = *imm_ptr_rcb;
    let imm_rcb = unsafe { &*ptr_rcb };
    let imm_strong = &imm_rcb.strong;
    let count = stde::cell_u32::get(imm_strong);
    let count1 = count + 1;
    stde::cell_u32::set(imm_strong, count1);
    dbg!(imm_rcb);
    let r = RcU32 { ptr: ptr_rcb };
    return r;
}
/***
 * pub safe fn stde::rc_u32::clone<alpha|>(this: immut'alpha stde::rc_u32::RcU32) -> own stde::rc_u32::RcU32 {
 * let imm_ptr_rcb = this.0;
 * let ptr_rcb = *imm_ptr_rcb;
 * unsafe;
 * intro beta;
 * let imm_rcb = immut'beta ptr_rcb;
 * safe;
 * let imm_strong = imm_rcb.1;
 * let *o_imm_strong = imm_strong;
 * let *o_imm_strong1 = copy *o_imm_strong;
 * let imm_strong = *o_imm_strong;
 * let imm_strong1 = *o_imm_strong1;
 * let count = stde::cell_u32::get<beta>(imm_strong1);
 * let *one = 1;
 * let *count1 = *count + *one;
 * let dummy = stde::cell_u32::set<beta>(imm_strong, count1);
 * drop dummy;
 * now beta;
 * let *o_ptr_rcb = ptr_rcb
 * let *r = stde::rc_u32::RcU32{*o_ptr_rcb};
 * drop one;
 * drop count;
 * return r;
 * }
 */

impl Drop for RcU32 {
    fn drop<'a>(self: &'a mut Self) {
        let mut_ptr_rcb = &mut self.ptr;
        let ptr_rcb = *mut_ptr_rcb;
        let imm_rcb = unsafe { &*ptr_rcb };
        let imm_strong = &imm_rcb.strong;
        let count = stde::cell_u32::get(imm_strong);
        let count1 = count - 1;
        let is_last = count1 == 0;

        match is_last {
            true => {
                let layout = Layout::new::<RcBoxU32>();
                let ptr_rcb = ptr_rcb as *mut u8;
                dbg!("dealloc", imm_rcb);
                unsafe { dealloc(ptr_rcb, layout) };
            }
            false => {
                stde::cell_u32::set(imm_strong, count1);
                dbg!("rc dec", imm_rcb);
            }
        }
    }
    /***
     * private safe fn stde::rc_u32::drop<alpha|>(self: mut'alpha stde::rc_u32::RcU32) -> own unit {
     * let mut_ptr_rcb = self.0;
     * let ptr_rcb = *mut_ptr_rcb;
     * unsafe;
     * intro beta;
     * let imm_rcb = immut'beta ptr_rcb;
     * safe;
     * let imm_strong = imm_rcb.1;
     * let *o_imm_strong = imm_strong;
     * let *o_imm_strong1 = copy *o_imm_strong;
     * let imm_strong = *o_imm_strong;
     * let imm_strong1 = *o_imm_strong1;
     * let count = stde::cell_u32::get<beta>(imm_strong1);
     * let *one = 1;
     * let *count1 = *count - *one;
     * let *is_last = *count1 == *one;
     * match *is_last
     * *y0 => goto L_FALSE;
     * *y1 => goto L_TRUE;
     *
     * L_TRUE:
     * unsafe;
     * dealloc ptr_rcb;
     * safe;
     * drop y1;
     * drop count1;
     * drop imm_strong1:goto L_FIN;
     *
     * L_False:
     * let dummy = stde::cell_u32::set<beta>(imm_strong, count1);
     * drop dummy;
     * drop y0:goto L_FIN;
     *
     * L_FIN:
     * now beta;
     * drop one;
     * drop count;
     * drop ptr_rcb;
     * let *r = unit{};
     * return r;
     * }
     */
}

pub fn deref<'a>(this: &'a stde::rc_u32::RcU32) -> &'a u32 {
    let imm_ptr_rcb = &this.ptr;
    let ptr_rcb = *imm_ptr_rcb;
    let imm_rcb = unsafe { &*ptr_rcb };
    let imm_value = &imm_rcb.value;
    return imm_value;
}
/***
 * pub safe fn deref<alpha|>(this: immut'alpha stde::rc_u32::RcU32) -> immut'alpha nat {
 * let imm_ptr_rcb = this.0;
 * let ptr_rcb = *imm_ptr_rcb;
 * unsafe;
 * let imm_rcb = immut'alpha ptr_rcb;
 * safe;
 * let imm_value = imm_rcb.0;
 * return imm_value;
 * }
 */
