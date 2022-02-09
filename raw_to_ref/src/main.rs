#![allow(unused)]
fn main() {
    let rmn;
    {
        let mut n: i32 = 42;
        let mn = &mut n;
        let imn = &mn;

        rmn = imn as *const &mut i32;
    }

    // mock(rmn);
}

fn mock(ptr: *const &mut i32) {}
