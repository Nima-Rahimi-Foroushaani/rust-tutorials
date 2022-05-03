use core::mem::{replace, zeroed};
use core::ptr;

pub struct BinaryHeap<T> {
    pub data: Vec<T>,
}

impl<T: Ord> BinaryHeap<T> {
    // T implements Ord

    pub fn sift_up(&mut self, start: usize, mut pos: usize) {
        unsafe {
            let new = replace(&mut self.data[pos], zeroed());
            // There is an element with all-zero byte-pattern
            // which is not necessarily valid
            while pos > start {
                let parent = (pos - 1) >> 1;
                // What if the '<=' panics!
                if new <= self.data[parent] {
                    break;
                }
                let x = replace(&mut self.data[parent], zeroed());
                ptr::write(&mut self.data[pos], x);
                pos = parent;
            }
            ptr::write(&mut self.data[pos], new);
        }
    }
}
