pub fn push_four<'a>(r: &'a mut Vec<i32>) {
    r.push(4)
}

pub fn access_types() {
    let mut v: Vec<i32> = vec![1, 2, 3]; // v is the owner
    {//----------------------------------------------------
        let mrv: &mut Vec<i32> = &mut v; //               |
        /***                                              |
         * mrv is a mutable borrow of v                   |
         * as long as this borrow is alive it        [mrv lifetim]
         * is not possible to access                      |
         * the vector through v                           |
         */ //                                            |
        push_four(mrv); // mutable borrow has full access |
    }//----------------------------------------------------

    let _ = v.pop(); // v has its ownership back

    {
        let srv: &Vec<i32> = &v;
        /***
         * srv is a shared/immutable borrow of v
         * the vector cannot get mutated as long as
         * it is borrowed by any immutable borrow
         */
        {
            let first: &i32 = v.first().unwrap();
            /***
             * multiple shared references, borrowing from
             * the same owner, can coexist
             */
            println!("{} is the first in {:?}", first, srv);
        }
    }
    let _ = v.pop();
    /***
     * The owner v goes out of scope here
     * and the value gets dropped
     */
}
