pub fn exclusive_own() {
    let mut v: Vec<i32> = vec![1, 2, 3]; // v is the owner
    {/***********************[l1]*************************/
        let mrv: &mut Vec<i32> = &mut v; // under [l1]    |
        push_four(mrv); //                                |
    }/****************************************************/
    let _ = v.pop(); // v has its ownership back
}
pub fn push_four<'a>(r: &'a mut Vec<i32>) {
    r.push(4)
}