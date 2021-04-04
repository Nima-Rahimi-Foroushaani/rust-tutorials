fn maximum(a: i32, b: i32) -> i32 {
    if a > b {
        a
    }
    else {
        b
    }
}

fn main() {
    let a = 42;
    let b = 24;

    let m = maximum(a, b);
    println!("The maximum of {} and {} is {}", a, b, m);
}
