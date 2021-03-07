fn main() {
    let x = u8::MAX;
    
match x.checked_add(1) {
    Some(x) => println!("The value of x is: {}", x),
    None => println!("Overflow"),
}

}