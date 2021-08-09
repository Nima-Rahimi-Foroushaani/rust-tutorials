mod access_simulate_fn;
mod simple_bool;
mod simple_i32;
mod simple_struct;
mod simple_unit;

fn main() {
    let n = 42;
    let res = simple_i32::simple_i32_shared_fail(&n);
    println!("result: {}", res);
}