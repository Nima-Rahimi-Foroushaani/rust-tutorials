struct Wrap<T> {
    wi32: T,
}

fn main()
//@ requires true;
/*@ ensures true;
@*/
{
    //@ assert true;
    let x = &mut Wrap::<i32> { wi32: 0 }.wi32;
    while *x < 42 {
        *x += 1;
    }
}
