#![allow(dead_code)]
struct CellU32 {
    v: u32,
}
/*@
predicate_ctor CellU32_nonatomic_borrow_content(void *l, thread_id_t t)() =
    CellU32_v(l, _);
// `shr` predicate for Cell<u32>
predicate CellU32_shared(lifetime_t k, thread_id_t t, void *l) =
    nonatomic_borrow(k, t, l, CellU32_nonatomic_borrow_content(l, t));
@*/

// VeriFast generates a contract like the following comment based on the function signature
// requires [?q]lifetime_token(?a) &*& thread_token(?t) &*& CellU32_shared(a, t, c);
// ensures [q]lifetime_token(a) &*& thread_token(t);
fn set<'a>(c: &'a CellU32, u: u32) {
    let p = &c.v as *const u32 as *mut u32;
    //@ open CellU32_shared(a, _t, c);
    //@ open_nonatomic_borrow(a, _t, c, _q_a);
    //@ open CellU32_nonatomic_borrow_content(c, _t)();
    unsafe {
        *p = u;
    }
    //@ close CellU32_nonatomic_borrow_content(c, _t)();
    //@ close_nonatomic_borrow();
}

fn get<'a>(c: &'a CellU32) -> u32 {
    //@ open CellU32_shared(a, _t, c);
    //@ open_nonatomic_borrow(a, _t, c, _q_a);
    //@ open CellU32_nonatomic_borrow_content(c, _t)();
    c.v
    //@ close CellU32_nonatomic_borrow_content(c, _t)();
    //@ close_nonatomic_borrow();
}
