fn main() {
    let x = u8::MAX;
    
    match x.checked_add(1) {
        Some(x) => println!("The value of x is: {}", x),
        None => println!("Overflow"),
    }
    
    let str = String::from("This is a string!");
    let str = &str[..];
    
    println!("The string \"{}\" is shadowed with a reference to itself!", str);

}