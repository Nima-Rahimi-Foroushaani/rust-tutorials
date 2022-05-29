pub struct Cell {
    value: i32,
}

impl Cell {
    pub fn new(value: i32) -> Cell {
        Cell { value }
    }

    pub fn get<'a>(&'a self) -> i32 {
        self.value
    }

    pub fn set<'a>(&'a self, n: i32) {
        let value_mut_ptr = &self.value as *const i32 as *mut i32;
        unsafe {
            *value_mut_ptr = n;
        }
    }
}
impl !Sync for Cell {}
