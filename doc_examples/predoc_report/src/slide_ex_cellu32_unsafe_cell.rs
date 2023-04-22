use core::cell::UnsafeCell;
use core::mem;

pub struct Cell<T: ?Sized> { value: UnsafeCell<T> }

impl<T> Cell<T> {
    pub const fn new(value: T) -> Cell<T> {
        Cell { value: UnsafeCell::new(value) }}

    pub fn replace(&self, val: T) -> T {
        mem::replace(unsafe { &mut *self.value.get() }, val)}
}
impl<T: Copy> Cell<T> {
    pub fn get(&self) -> T {
        unsafe { *self.value.get() }}
}

impl<T: ?Sized> !Sync for Cell<T> {}
