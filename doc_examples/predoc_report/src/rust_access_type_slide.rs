pub fn push_four<'a>(r: &'a mut Vec<i32>) {
    r.push(4)
}
/*** [l1] means the lifetime l1 */
pub fn access_types() {
    let mut v: Vec<i32> = vec![1, 2, 3]; // v is the owner
    {/***********************[l1]*************************/
        let mrv: &mut Vec<i32> = &mut v; // under [l1]    |
        push_four(mrv); //                                |
    }/****************************************************/

    let _ = v.pop(); // v has its ownership back

    {/***********************[l2]*************************/
        let srv: &Vec<i32> = &v; // under [l2]            |
        {/*******************[l3]*****************/       |
            let first: &i32 = // under [l3]       |       |
                v.first().unwrap(); //            |       |
            println!("{} is the first in {:?}", //|       |
                first, srv); //                   |       |
        }/****************************************/       |
    }/****************************************************/
    let _ = v.pop(); // v has its ownership back
    /***
     * The owner v goes out of scope
     * vector gets dropped
     */
}
