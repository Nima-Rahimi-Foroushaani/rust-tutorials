pub fn temp_exclusive_own() {
    let mut v: Vec<i32> = vec![1, 2, 3]; // v is the owner
    {/***********************[l1]*************************/
        let mrv: &/*'l1*/mut Vec<i32> = &mut v;           |
        push_four(mrv); //                                |
    }/****************************************************/
    let _ = v.pop(); // v has its ownership back
}
pub fn push_four<'a>(r: &'a mut Vec<i32>) {
    r.push(4)
}