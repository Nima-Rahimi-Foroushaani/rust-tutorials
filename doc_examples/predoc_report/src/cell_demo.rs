#![allow(dead_code)]
struct CellU32 {
    v: u32,
}

/*@
predicate CellU32_owned(uint32_t v, thread_id_t t) = true;

// Generated by VeriFast Automatically
//predicate_ctor CellU32_full_borrow_content(void *l, thread_id_t t)() =
//    CellU32_v(l, ?v) &*& CellU32_owned(v, t);

predicate_ctor CellU32_nonatomic_borrow_content(void *l, thread_id_t t)() =
    CellU32_v(l, _);

// `shr` predicate for Cell<u32>
predicate CellU32_shared(lifetime_t k, thread_id_t t, void *l) =
    [_]nonatomic_borrow(k, t, l, CellU32_nonatomic_borrow_content(l, t));

// Proof Obligations
lemma void CellU32_shared_mono(lifetime_t k, lifetime_t k1, thread_id_t t, void *l)
    requires lifetime_inclusion(k1, k) == true &*& [_]CellU32_shared(k, t, l);
    ensures [_]CellU32_shared(k1, t, l);
{
  open CellU32_shared(k, t, l);
  nonatomic_borrow_mono(k, k1, t, l, CellU32_nonatomic_borrow_content(l, t));
  close CellU32_shared(k1, t, l);
  leak CellU32_shared(k1, t, l);
}

lemma void CellU32_share(lifetime_t k, thread_id_t t, void *l)
    requires full_borrow(k, CellU32_full_borrow_content(l, t)) &*& [?q]lifetime_token(k);
    ensures [_]CellU32_shared(k, t, l) &*& [q]lifetime_token(k);
{
    produce_lemma_function_pointer_chunk implies(CellU32_full_borrow_content(l, t), CellU32_nonatomic_borrow_content(l, t))() {
        open CellU32_full_borrow_content(l, t)();
        open CellU32_owned(_, t);
        close CellU32_nonatomic_borrow_content(l, t)();
    } {
        produce_lemma_function_pointer_chunk implies(CellU32_nonatomic_borrow_content(l, t), CellU32_full_borrow_content(l, t))() {
            open CellU32_nonatomic_borrow_content(l, t)();
            assert CellU32_v(l, ?v);
            close CellU32_owned(v, t);
            close CellU32_full_borrow_content(l, t)();
        } {
            full_borrow_implies(k, CellU32_full_borrow_content(l, t), CellU32_nonatomic_borrow_content(l, t));
        }
    }
    full_borrow_into_nonatomic_borrow(k, t, l, CellU32_nonatomic_borrow_content(l, t));
    close CellU32_shared(k, t, l);
    leak CellU32_shared(k, t, l);
}
@*/

fn new(u: u32) -> CellU32
// requires thread_token(?_t);
// ensures thread_token(_t) &*& CellU32(result.v, _t);
{
    CellU32 { v: u }
    //@ close CellU32_owned(u, _t);
}

fn set<'a>(c: &'a CellU32, u: u32)
// requires [?_q_a]lifetime_token(?a) &*& thread_token(?_t) &*& CellU32_shared(a, t, c);
// ensures [_q_a]lifetime_token(a) &*& thread_token(_t);
{
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

fn get<'a>(c: &'a CellU32) -> u32
// requires [?_q_a]lifetime_token(?a) &*& thread_token(?_t) &*& CellU32_shared(a, t, c);
// ensures [_q_a]lifetime_token(a) &*& thread_token(_t);
{
    //@ open CellU32_shared(a, _t, c);
    //@ open_nonatomic_borrow(a, _t, c, _q_a);
    //@ open CellU32_nonatomic_borrow_content(c, _t)();
    c.v
    //@ close CellU32_nonatomic_borrow_content(c, _t)();
    //@ close_nonatomic_borrow();
}
