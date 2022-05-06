use core::mem::{replace, MaybeUninit};
use core::ptr;

pub struct BinaryHeap<T> {
    pub data: Vec<T>,
}

impl<T: Ord> BinaryHeap<T> {
    // T implements Ord

    pub fn sift_up(&mut self, start: usize, mut pos: usize) {
        unsafe {
            let new = replace(
                &mut self.data[pos],
                MaybeUninit::<T>::zeroed().assume_init(),
            );
            // There is an element with all bytes zeroed
            // which is not necessarily a valid value
            while pos > start {
                let parent = (pos - 1) >> 1;
                if new <= self.data[parent] {
                    // What if the '<=' panics!
                    break;
                }
                let x = replace(
                    &mut self.data[parent],
                    MaybeUninit::<T>::zeroed().assume_init(),
                );
                ptr::write(&mut self.data[pos], x);
                pos = parent;
            }
            ptr::write(&mut self.data[pos], new);
        }
    }
}
