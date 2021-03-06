fn main() {
    let mut s: String = String::from("Hello Slices!");
    
    let sl1 = &mut s[..5];
    let sl2 = &mut s[6..];
    
    println!("{}, {}", sl1, sl2);
}
