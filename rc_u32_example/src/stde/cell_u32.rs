#[derive(Debug)]
pub struct CellNat {
    value: u32,
}
// pub struct stde::cell_nat::CellNat<>{nat}

pub fn new(v: u32) -> CellNat {
    let r = CellNat { value: v };
    return r;
}
/***
 * pub safe fn stde::cell_nat::new<|>(v: own nat) -> own stde::cell_nat::CellNat<> {
 * let *r = stde::cell_nat::CellNat<>{*v};
 * return r;
 * }
 */

pub fn get<'a>(this: &'a CellNat) -> u32 {
    let imm_value = &this.value;
    let r = *imm_value; //copy
    return r;
}
/***
 * pub safe fn stde::cell_nat::get<alpha|>(this: immut'alpha stde::cell_nat::CellNat<>) -> own nat {
 * let {*imm_value} = *this;
 * let *r = copy *imm_value;
 * drop imm_value;
 * return r;
 * }
 */

pub fn set<'a>(this: &'a CellNat, v: u32) -> () {
    let imm_value = &this.value;
    let ptr_value = imm_value as *const u32 as *mut u32;
    let mut_value = unsafe { &mut *ptr_value };
    dbg!(imm_value); // imm_value and mut_value are alive together
    *mut_value = v;
}
/***
 * pub safe fn stde::cell_nat::set<alpha|>(this: immut'alpha stde::cell_nat::CellNat<>, v: own nat) -> own unit {
 * // producing parameters
 * {{sstore=this:#this,v:#v; sheap=alpha::[shr(#fr)]#this->stde::cell_nat::CellNat<>,[del]#v->nat(#value)}}
 *
 * let {*imm_value} = this; //type access check is performed at type checking
 * // pointer calculation and openning the type predicate
 * // alpha::[shr(fr)]this->stde::cell_nat::CellNat<> =* alpha::[shr(1.0)]this->nat(value) *
 * No active path to #this with type nat unless frozen by alpha
 * {{sstore=this:#this,v:#v,imm_value:#this; sheap=[del]#v->nat(#value),alpha::[shr(1.0)]#this->nat(#value1)}}
 *
 * let ptr_value = raw imm_value;
 * {{sstore=this:#this,v:#v,imm_value:#this,ptr_value:#this;
 * sheap=alpha::[shr(#fr)]#this->stde::cell_nat::CellNat<>,[del]#v->#value}}
 *
 * unsafe;
 * intro beta;
 * let mut_value = mutbor'beta ptr_value;
 *
 * safe;
 * swap(*mut_value, *v);
 * drop mut_value;
 * now beta;
 * drop ptr_value;
 * drop imm_value;
 * drop v;
 * let *r = unit<>{};
 * return r;
 * }
 */
