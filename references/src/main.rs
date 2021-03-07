fn main() {
    let mut v:Vec<i32> = vec![1,2,3,4,5];
    let r1 = &mut v;
    let len1 = r1.len();

    let r2 = &mut v;
    r2.pop().expect("Panic!");
    
    for i in 0..len1 {
        println!("{}", r1[i]);
    }
    
    for x in r2 {
        println!("{}", x);
    }
}