use std::ptr::addr_of_mut;
//@ #include <listex.gh>

struct Node {
    prev: *mut Node,
    value: i32,
    next: *mut Node,
}

/*@
predicate nodes(struct Node *before, struct Node *first, struct Node *last, struct Node *after; list<int> elements) =
    first == after ?
        last == before &*& elements == nil
    :
        first->prev |-> before &*&
        first->value |-> ?value &*&
        first->next |-> ?next &*&
        malloc_block_Node(first) &*&
        nodes(first, next, last, after, ?elements0) &*&
        elements == cons(value, elements0);

predicate deque(struct Node *sentinel; list<int> elements) =
    sentinel->prev |-> ?last &*&
    sentinel->value |-> _ &*&
    sentinel->next |-> ?first &*&
    malloc_block_Node(sentinel) &*&
    nodes(sentinel, first, last, sentinel, elements);

@*/

unsafe fn create_deque() -> *mut Node
//@ requires true;
//@ ensures deque(result, nil);
{
    let layout = std::alloc::Layout::new::<Node>();
    let sentinel: *mut Node = std::alloc::alloc(layout) as *mut Node;
    if sentinel.is_null() {
        std::alloc::handle_alloc_error(layout)
    }
    addr_of_mut!((*sentinel).prev).write(sentinel);
    addr_of_mut!((*sentinel).next).write(sentinel);
    return sentinel;
}

unsafe fn push_front(deque: *mut Node, value: i32)
//@ requires deque(deque, ?values);
//@ ensures deque(deque, cons(value, values));
{
    let layout = std::alloc::Layout::new::<Node>();
    let n: *mut Node = std::alloc::alloc(layout) as *mut Node;
    if n.is_null() {
        std::alloc::handle_alloc_error(layout)
    }
    addr_of_mut!((*n).prev).write(deque);
    addr_of_mut!((*n).value).write(value);
    addr_of_mut!((*n).next).write((*deque).next);
    (*(*n).prev).next = n;
    //@ open nodes(deque, ?first, ?last, deque, values);
    (*(*n).next).prev = n;
}

/*@

lemma void deque_snoc()
    requires nodes(?before, ?first, ?last, ?after, ?values) &*& first != after;
    ensures
        nodes(before, first, ?last0, last, ?values0) &*&
        last->prev |-> last0 &*& last->value |-> ?value &*& last->next |-> after &*&
        malloc_block_Node(last) &*& values == append(values0, {value});
{
    open nodes(before, first, last, after, values);
    assert nodes(first, ?next, last, after, ?values1);
    if (next == after) {
        open nodes(first, next, last, after, values1);
        close nodes(before, first, before, last, nil);
    } else {
        deque_snoc();
        assert nodes(first, next, ?last0, last, ?values0);
        close nodes(before, first, last0, last, cons(head(values), values0));
    }
}

lemma void deque_add()
    requires
        nodes(?before, ?first, ?last, ?after, ?values) &*&
        after->prev |-> last &*&
        after->value |-> ?value &*&
        after->next |-> ?next &*& next->value |-> ?nextValue &*&
        malloc_block_Node(after);
    ensures
        nodes(before, first, after, next, append(values, {value})) &*& next->value |-> nextValue;
{
    open nodes(before, first, last, after, values);
    if (first == after) {
        close nodes(after, next, after, next, nil);
        close nodes(last, after, after, next, {value});
    } else {
        deque_add();
        close nodes(before, first, after, next, append(values, {value}));
    }
}

@*/

unsafe fn push_back(deque: *mut Node, value: i32)
//@ requires deque(deque, ?values);
//@ ensures deque(deque, append(values, {value}));
{
    //@ open deque(_, _);
    //@ bool empty = deque->next == deque;
    /*@
    if (empty) {
        open nodes(deque, _, _, _, _);
    } else {
        deque_snoc();
    }
    @*/
    let layout = std::alloc::Layout::new::<Node>();
    let n: *mut Node = std::alloc::alloc(layout) as *mut Node;
    if n.is_null() {
        std::alloc::handle_alloc_error(layout)
    }
    addr_of_mut!((*n).prev).write((*deque).prev);
    addr_of_mut!((*n).value).write(value);
    addr_of_mut!((*n).next).write(deque);
    (*(*n).prev).next = n;
    (*(*n).next).prev = n;
    /*@
    if (empty) {
    } else {
        deque_add();
        deque_add();
    }
    @*/
}

unsafe fn is_empty(deque: *mut Node) -> bool
//@ requires deque(deque, ?values);
//@ ensures deque(deque, values) &*& result == (values == nil);
{
    //@ open deque(deque, values);
    //@ open nodes(?before, ?first, ?last, ?after, values);
    return (*deque).next == deque;
}

// precondition: deque is not empty
unsafe fn pop_front(deque: *mut Node) -> i32
//@ requires deque(deque, cons(?value, ?values));
//@ ensures deque(deque, values) &*& result == value;
{
    let n = (*deque).next;
    //@ open nodes(deque, n, ?last, deque, _);
    let result = (*n).value;
    (*(*n).prev).next = (*n).next;
    //@ open nodes(n, ?next, last, deque, _);
    (*(*n).next).prev = (*n).prev;
    std::alloc::dealloc(n as *mut u8, std::alloc::Layout::new::<Node>());
    return result;
}

// precondition: deque is not empty
unsafe fn pop_back(deque: *mut Node) -> i32
//@ requires deque(deque, ?values) &*& values != nil;
//@ ensures deque(deque, ?values1) &*& values == append(values1, {result});
{
    let n = (*deque).prev;
    //@ open nodes(deque, ?first, ?last, ?after, values);
    //@ close nodes(deque, first, last, after, values);
    //@ deque_snoc();
    let result = (*n).value;
    //@ bool empty = deque->next == n;
    /*@
    if (empty) {
        open nodes(deque, first, _, _, _);
    } else {
        deque_snoc();
    }
    @*/
    (*(*n).prev).next = (*n).next;
    (*(*n).next).prev = (*n).prev;
    /*@
    if (empty) {
    } else {
        deque_add();
    }
    @*/
    std::alloc::dealloc(n as *mut u8, std::alloc::Layout::new::<Node>());
    return result;
}

// precondition: the deque is empty
unsafe fn drop_deque(deque: *mut Node)
//@ requires deque(deque, nil);
//@ ensures true;
{
    std::alloc::dealloc(deque as *mut u8, std::alloc::Layout::new::<Node>());
    //@ open nodes(_, _, _, _, _);
}

unsafe fn main_()
//@ requires true;
//@ ensures true;
{
    let deque = create_deque();
    assert!(is_empty(deque));
    push_front(deque, 10);
    assert!(!is_empty(deque));
    push_back(deque, 20);
    push_front(deque, 30);
    push_back(deque, 40);
    assert!(pop_front(deque) == 30);
    assert!(pop_back(deque) == 40);
    assert!(pop_front(deque) == 10);
    assert!(pop_back(deque) == 20);
    drop_deque(deque);
}

fn main()
//@ requires true;
//@ ensures true;
{
    unsafe {
        main_();
    }
}
