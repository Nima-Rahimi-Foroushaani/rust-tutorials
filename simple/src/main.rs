// struct Wrap<T> {
//     wi32: T,
// }
struct Something();

fn main()
//@ requires true;
/*@ ensures true;
@*/
{
    //@ assert true;
    let x = &mut 0; //&mut Wrap::<i32> { wi32: 0 }.wi32;
    while *x < 42 {
        *x += 1;
    }
}
