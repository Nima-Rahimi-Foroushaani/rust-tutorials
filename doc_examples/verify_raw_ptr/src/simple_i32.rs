use crate::access_simulate_fn;

#[allow(unused)]
pub fn simple_i32_shared() {
    let x: i32;
    x = 42;
    let shared_x: &i32;
    shared_x = &x;
    
    access_simulate_fn::read_access(shared_x);
}