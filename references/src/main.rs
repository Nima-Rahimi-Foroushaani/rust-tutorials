fn main() {
    let mut v:Vec<i32> = vec![1,2,3,4,5];
    // first mutable reference
    let r1 = &mut v;
    let len1 = r1.len();

    // second mutable reference - overlaps with the scope of the first -> Error
    let r2 = &mut v;
    r2.pop().expect("Panic!");
    
    // Below for may be a motivation for applying mutable reference constraints even in a single thread
    for i in 0..len1 {
        println!("{}", r1[i]);
    }
    
    for x in r2 {
        println!("{}", x);
    }
}
