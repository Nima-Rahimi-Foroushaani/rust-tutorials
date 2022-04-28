pub fn access_types() {
    let mut x = vec![1..10]; // x is the owner
    {
        let x_mut_ref = &mut x;
        /***
         * x_mut_ref is a mutable borrow of x
         * as long as this borrow is alive it
         * is not possible to access vector through x
         */
    }

    {
        let x_shr_ref = &x;
        /***
         * x_shr_ref is a shared/immutable borrow of x
         * vector cannot get mutated through x as long as
         * this borrow is alive
         */
    }
    /***
     * The owner, x goes out of scope here
     * and the value gets dropped
     */
}
