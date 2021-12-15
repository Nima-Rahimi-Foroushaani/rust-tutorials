mod stde;

#[allow(unused)]
fn main() -> () {
    let rc1 = stde::rc_u32::new(42);
    {
        let imm_rc1 = &rc1;
        let rc2 = stde::rc_u32::RcU32::clone(imm_rc1);
    }
    let rc3 = rc1; // move
}
/***
 * private safe fn main<|>() -> own unit {
 * let *num = 42;
 * let rc1 = stde::rc_u32::new<>(num);
 *
 * intro alpha;
 * let imm_rc1 = mutbor'alpha rc1;
 * immut imm_rc1;
 * let rc2 = stde::rc_u32::clone<alpha>(imm_rc1);
 * drop rc2;
 * now alpha;
 *
 * let *o_rc1 = rc1;
 * let rc3 = *o_rc1;
 * drop rc3;
 *
 * let *r = unit{};
 * return r;
 * }
 */
