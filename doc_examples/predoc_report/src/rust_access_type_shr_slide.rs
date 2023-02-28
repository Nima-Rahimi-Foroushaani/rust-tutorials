pub fn shared_own() {
    let mut v: Vec<i32> = vec![1, 2, 3]; // v is the owner
    {/***********************[l2]*************************/
        let srv: &/*'l2*/Vec<i32> = &v;//                 |
        {/*******************[l3]*****************///     |
            let first: &/*'l3*/i32 =//            |       |
                v.first().unwrap(); //            |       |
            println!("{} is the first in {:?}",// |       |
                first, srv); //                   |       |
        }/****************************************///     |
    }/****************************************************/
    let _ = v.pop(); // v has its ownership back
}