#![allow(unused)]

#[derive(Debug)]
struct Person {
    sex : String,
    age : u8,
    name: String
}

fn own_transfer_vv() {
    let mut p1 = Person{sex: "male".to_string(), age: 30, name: "John".to_string()};
    let mut p2 = Person{sex: "male".to_string(), age: 31, name: "Tom".to_string()};
    p1 = p2;
    println!("{:?}", p1);
//    println!("{:?}", p2);
    let mut a1 = 100;
    let mut a2 = 200;
    a1 = a2;
    println!("{:?}", a1);
    println!("{:?}", a2);
}

fn own_transfer_vr() {
    let mut p1 = Person{sex: "male".to_string(), age: 30, name: "John".to_string()};
    let mut p2 = Person{sex: "male".to_string(), age: 31, name: "Tom".to_string()};
    let rp2 = & mut p2;
//    p1 = *rp2;
    println!("{:?}", p1);
//    println!("{:?}", p2);
    let mut a1 = 100;
    let mut a2 = 200;
    let ra2 = & mut a2;
    a1 = *ra2;
    println!("{:?}", a1);
    println!("{:?}", a2);
}

fn own_transfer_rv() {
    let mut p1 = Person{sex: "male".to_string(), age: 30, name: "John".to_string()};
    let mut p2 = Person{sex: "male".to_string(), age: 31, name: "Tom".to_string()};
    let rp1 = & mut p1;
    *rp1 = p2;
    println!("{:?}", p1);
//    println!("{:?}", p2);
    let mut a1 = 100;
    let mut a2 = 200;
    let ra1 = & mut a1;
    *ra1 = a2;
    println!("{:?}", a1);
    println!("{:?}", a2);
}

fn own_transfer_rr() {
    let mut p1 = Person{sex: "male".to_string(), age: 30, name: "John".to_string()};
    let mut p2 = Person{sex: "male".to_string(), age: 31, name: "Tom".to_string()};
    let rp1 = & mut p1;
    let rp2 = & mut p2;
//    *rp1 = *rp2;
    println!("{:?}", p1);
//    println!("{:?}", p2);
    let mut a1 = 100;
    let mut a2 = 200;
    let ra1 = & mut a1;
    let ra2 = & mut a2;
    *ra1 = *ra2;
    println!("{:?}", a1);
    println!("{:?}", a2);
}

fn main() {
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
    
    own_transfer_vv();
    own_transfer_vr();
    own_transfer_rv();

}


fn own_in_heap() {
    let name = Box::new(String::from("Someone"));
}