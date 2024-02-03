pub fn exclusive_own() {
    let mut v: Vec<i32> = vec![1, 2, 3];// v is the owner
    let mut v1 = push_four(v);
    let _ = v1.pop();
    // v1 gets dropped
}
pub fn push_four(mut v: Vec<i32>) -> Vec<i32> {
    v.push(4); return v;
}