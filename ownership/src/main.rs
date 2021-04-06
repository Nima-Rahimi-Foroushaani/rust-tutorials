#[allow(unused)]

fn main() {
    
    #[derive(Debug)]
    struct Person {
        sex : String,
        age : u8,
        name: String
    }
    
    let p1 = Person{
        sex : String::from("Female"),
        age : 30,
        name: String::from("Alice")
        };
    
    let mut p2 = p1; // move
    p2.age += 1;
    
    println!("a mutable variable ownes value:{:?}", p2);
    
    let p2mr = &mut p2;
    
    // p2.age += 1; // it is mutably borrowed and we cannot modify it
    
    println!("a mutable reference to p2 {:?}", p2mr);
    
    // because the lifetime of reference has ended 
    p2.age += 1;
    
    let p2r = &p2;
    
    // p2.age += 1; // it is immutably borrowed and we cannot modify it
    println!("a immutable reference:{:?}", p2r);
}
