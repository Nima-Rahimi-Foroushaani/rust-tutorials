use std::ptr::addr_of_mut;

pub struct Node {
    prev: *mut Node,
    value: i32,
    next: *mut Node,
}

pub unsafe fn create_deque() -> *mut Node {
    let sentinel: *mut Node = std::alloc::alloc(std::alloc::Layout::new::<Node>()) as *mut Node;
    addr_of_mut!((*sentinel).prev).write(sentinel);
    addr_of_mut!((*sentinel).next).write(sentinel);
    return sentinel;
}

pub unsafe fn is_empty(deque: *mut Node) -> bool {
    return (*deque).next == deque;
}
// ...
pub fn caller() {
    unsafe {
        let deque = create_deque();
        assert!(is_empty(deque));
    }
}
