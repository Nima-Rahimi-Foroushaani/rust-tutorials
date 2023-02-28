pub fn shared_own() {
    let mut v: Vec<i32> = vec![1, 2, 3];// v is the owner
    {/***********************[l1]*************************/
        let srv: &/*'l1*/Vec<i32> = &v;//                 |
        {/*******************[l2]*****************///     |
            let first: &/*'l2*/i32 =//            |       |
                v.first().unwrap();//             |       |
            println!("{} is the first in {:?}",// |       |
                first, srv);//                    |       |
        }/****************************************///     |
    }/****************************************************/
    let _ = v.pop();// v has its ownership back
}