pub fn access_types()
{
    let mut v: Vec<i32> = vec![1, 2, 3]; // v is the owner
    {
        let mut_ref: &mut Vec<i32> = &mut v;
        /***
         * mut_ref is a mutable borrow of v
         * as long as this borrow is alive it
         * is not possible to access the vector through v
         */
        mut_ref.push(4); // mutable borrow has full access
    }

    let _ = v.pop();

    {
        let shr_ref: &Vec<i32> = &v;
        /***
         * shr_ref is a shared/immutable borrow of v
         * the vector cannot get mutated as long as
         * it is borrowed by any immutable borrow
         */
        {
            let first: &i32 = v.first().unwrap();
            /***
             * multiple shared references borrowing from
             * the same owner can coexist
             */
            println!("{} is the first in {:?}", first, shr_ref);
        }
    }
    let _ = v.pop();
    /***
     * The owner, v goes out of scope here
     * and the value gets dropped
     */
}
