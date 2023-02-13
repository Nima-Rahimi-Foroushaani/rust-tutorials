#![allow(dead_code)]
struct CellU32 {
    v: u32,
}
/*@
predicate_ctor CellU32_nonatomic_borrow_content(void *l, thread_id_t t)() =
    CellU32_v_(l, _);
// `shr` predicate for Cell<u32>
predicate CellU32_shared(lifetime_t k, thread_id_t t, void *l) =
    nonatomic_borrow(k, t, l, CellU32_nonatomic_borrow_content(l, t));
@*/

fn set<'a>(c: &'a CellU32, u: u32)
//@ requires [?q]lifetime_token(?a) &*& thread_token(?t) &*& CellU32_shared(a, t, c);
//@ ensures [q]lifetime_token(a) &*& thread_token(t);
{
    let p = &c.v as *const u32 as *mut u32;
    //@ open CellU32_shared(a, t, c);
    //@ open_nonatomic_borrow(a, t, c, q);
    //@ open CellU32_nonatomic_borrow_content(c, t)();
    unsafe {
        *p = u;
    }
    //@ close CellU32_nonatomic_borrow_content(c, t)();
    //@ close_nonatomic_borrow();
}
