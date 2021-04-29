enum List<'a,T> {
    Nil,
    Cons(T, &'a List<'a, T>)
    
} 

fn main() {
    let n: List<'_, i32> = List::Nil;
    let l = List::Cons(10, &n);
    
    match l {
        List::Nil=> {
            println!{"Empty"};
        }
        List::Cons(i, _)=> {
            println!("{}", i)
        }
    }
    println!("Hello, world!");
}
