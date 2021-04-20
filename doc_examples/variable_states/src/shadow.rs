#![allow(unused)]

pub fn f1() {
    let x: i32 = 42;
    //{ start of the scope of x : f32
    let x: f32 = std::f32::consts::PI; // shadows x:i32
    
    // Inner scope
    {
        let x: String; // shadows x:f32
        //println!("{}", x);
        //error[E0381]: borrow of possibly-uninitialized variable: `x`
    }
    
    println!("{}", x); // prints PI
    
    //} end of the scope of x: f32
    // and immediately end of the scope of x: i32 as the function ends
}