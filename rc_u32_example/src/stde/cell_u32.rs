pub struct CellU32 {
    value: u32,
}
// pub struct stde::cell_u32::CellU32{nat}

pub fn new(v: u32) -> CellU32 {
    let r = CellU32 { value: v };
    return r;
}
/***
 * pub safe fn stde::cell_u32::new<|>(v: own nat) -> own stde::cell_u32::CellU32 {
 * let *r = stde::cell_u32::CellU32{*v};
 * return r;
 * }
 */

pub fn get<'a>(this: &'a CellU32) -> u32 {
    let imm_value = &this.value;
    let r = *imm_value; //copy
    return r;
}
/***
 * pub safe fn stde::cell_u32::get<alpha|>(this: immut'alpha stde::cell_u32::CellU32) -> own nat {
 * let imm_value = this.0;
 * let *r = copy *imm_value;
 * drop imm_value;
 * return r;
 * }
 */

pub fn set<'a>(this: &'a CellU32, v: u32) -> () {
    let imm_value = &this.value;
    let ptr_value = imm_value as *const u32 as *mut u32;
    let mut_value = unsafe { &mut *ptr_value };
    *mut_value = v;
}
/***
 * pub safe fn stde::cell_u32::set<alpha|>(this: immut'alpha stde::cell_u32::CellU32, v: own nat) -> own unit {
 * let imm_value = this.0;
 * let ptr_value = raw imm_value;
 * drop imm_value;
 * unsafe;
 * intro beta;
 * let mut_value = mut'beta ptr_value;
 * safe;
 * swap(*mut_value, *v);
 * drop mut_value;
 * now beta;
 * drop ptr_value;
 * drop v;
 *
 * let *r = unit{};
 * return r;
 * }
 */
