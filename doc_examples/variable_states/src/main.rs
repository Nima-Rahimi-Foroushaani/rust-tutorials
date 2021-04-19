mod unbound;
mod borrow;
mod shadow;

fn main() {
    use unbound::{f1, f2, f3};
f1();   
f2();
f3(); 

borrow::f1();
borrow::f2();

shadow::f1();
}
