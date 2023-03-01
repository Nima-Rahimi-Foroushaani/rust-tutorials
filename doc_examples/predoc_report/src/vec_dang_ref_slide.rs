fn func() {
let mut v = vec![1, 2, 3];
let first: &Vec<i32> = v.first().unwrap();
v.push(4);
//cannot borrow `v` as mutable because it is also borrowed as immutable
println!("{}", first);
}
