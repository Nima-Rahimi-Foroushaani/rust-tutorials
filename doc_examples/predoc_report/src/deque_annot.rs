use std::ptr::addr_of_mut;

pub struct Node {
    prev: *mut Node,
    value: i32,
    next: *mut Node,
}

unsafe fn create_deque() -> *mut Node
//@ requires true;
/*
 @ ensures malloc_block_Node(sentinel) &*& Node_prev(result, result) &*&
 Node_value(result, _) &*& Node_next(result, result);
 */
{
    let sentinel: *mut Node = std::alloc::alloc(std::alloc::Layout::new::<Node>()) as *mut Node;
    addr_of_mut!((*sentinel).prev).write(sentinel);
    addr_of_mut!((*sentinel).next).write(sentinel);
    return sentinel;
}
