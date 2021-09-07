#![allow(unused)]
fn main() {
    /*let mut v:Vec<i32> = vec![1,2,3,4,5];
    // first mutable reference
    let r1 = &mut v;
    let len1 = r1.len();

    // second mutable reference - overlaps with the scope of the first -> Error
    let r2 = &mut v;
    r2.pop().expect("Panic!");
    
    // Below for loop may be a motivation for applying mutable reference constraints even in a single thread
    for i in 0..len1 {
        println!("{}", r1[i]);
    }
    
    for x in r2 {
        println!("{}", x);
    }*/
    
    function1();
    function2();
}

/** Here we see the type system not only takes care of the end of the borrower lifetime,
 * but also takes new assignments to the borrower into account*/
fn function1() {
    let mut i1 = 42;
    let mut ri: &i32 = &i1;
    let i2 = 43;
    ri = &i2;
    i1 = 0;
    
    println!("{},{},{}",i1,i2,ri)
}

/** Here we see interleaved lifetimes */
fn function2() {
    let mut i1 = 42;
    let mut i2 = 43;
    let ri1 = &i1;
    let ri2 = &i2;
    println!("{}",ri1);
    i1 = 0;
    println!("{}",ri2);
}

fn function3() {
    let x = 42;
    let rx1 = &x;
    let rx2 = &x;
}