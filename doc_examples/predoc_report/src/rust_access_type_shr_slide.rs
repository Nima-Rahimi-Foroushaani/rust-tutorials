pub fn shared_own() {
    let mut v: Vec<i32> = vec![1, 2, 3]; // v is the owner
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
}