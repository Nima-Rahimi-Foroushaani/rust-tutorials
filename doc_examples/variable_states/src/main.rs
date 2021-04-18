#[allow(unused)]
fn main() {
    //scope 1
    {
        let x:i32;
        //println!("{}", x); //error[E0381]: borrow of possibly-uninitialized variable: `x`
        //let y = x; //error[E0381]: use of possibly-uninitialized variable: `x`
        x = 42; // shadows first x
        println!("{}", x);
    }
    
    //scope 2
    {
        let  str = String::from("Hello");
        let str1 = str;
        //println!("{}", str); //error[E0382]: borrow of moved value: `str`
        //str = String::from("world"); // error[E0384]: cannot assign twice to immutable variable `str`
        println!("{}", str1);
    }
    
    //scope 3
    {
        let mut  str = String::from("Hello");
        let str1 = str;
        str = String::from("world");
        println!("{} {}", str1, str);
    }
}
