#![allow(unused)]
fn main() {
    /*let mut v:Vec<i32> = vec![1,2,3,4,5];
    // first mutable reference
    let r1 = &mut v;
    let len1 = r1.len();

    // second mutable reference - overlaps with the scope of the first -> Error
    let r2 = &mut v;
    r2.pop().expect("Panic!");
    // Below for loop may be a motivation for applying mutable reference constraints even in a single thread
    for i in 0..len1 {
        println!("{}", r1[i]);
    }
    for x in r2 {
        println!("{}", x);
    }*/

    // function1();
    // function2();
    // function8();
    ref_static();
    ref_dyn();
}

/** Here we see the type system not only takes care of the end of the borrower lifetime,
 * but also takes new assignments to the borrower into account*/
fn function1() {
    let mut i1 = 42;
    let mut ri: &i32 = &i1;
    let i2 = 43;
    ri = &i2;
    i1 = 0;

    println!("{},{},{}", i1, i2, ri)
}

/** Here we see interleaved lifetimes */
fn function2() {
    let mut i1 = 42;
    let mut i2 = 43;
    let ri1 = &i1;
    let ri2 = &i2;
    println!("{}", ri1);
    i1 = 0;
    println!("{}", ri2);
}

fn function3() {
    let x = 42;
    let rx1 = &x;
    let rx2 = &x;
}

fn function4() -> i32 {
    //    let y =
    {
        let mut x: i32 = 42;
        &x
    };
    43
}

fn function5() {
    let mut x: i32 = 42;
    let rx: &mut i32 = &mut x;
    let rrx: &&mut i32 = &rx;
    //    *rx = 43;
    println!("{}", rrx);
}

fn function6() {
    let mut x = 42;
    let rx = &mut x;
    *rx = 0;
    println!("{}", *rx);
}

fn function7() {
    let mut x: i32 = 42;
    let mut rx: &mut i32 = &mut x;
    let rrx: &mut &mut i32 = &mut rx;
    let mut y: i32 = 43;
    *rrx = &mut y;
    //    x=24;//cannot assign to `x` because it is borrowed
    println!("{}", **rrx);
}

fn function8() {
    let mut x: i32 = 1;
    let mut rx: &mut i32 = &mut x;

    let rrx: &mut &mut i32 = &mut rx;
    let rxp: &mut i32 = *rrx;

    // println!("{} {}", rrx, rxp);
}

struct Container<'a> {
    r: &'a mut i32,
}
fn ref_static() {
    let mut x = 42;
    let c = Container { r: &mut x };
    let ic = &c;
    // *c.r = 43; fails
    println!("{}", ic.r);
    *c.r = 43;
    println!("{}", c.r)
}
use std::rc::Rc;

fn ref_dyn() {
    let mut x = 42;

    let rc_mx = Rc::new(&mut x);
    let rc_mx1 = rc_mx.clone();

    // **rc_mx = 43; fails
    // trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<&mut i32>`
    // This has been taken care by std lib developers. Here They could make a mistake and implement
    // that Trait for "Rc" which would break type system guarantees
    println!("{} {}", rc_mx, rc_mx1)
}

fn passive<'a>(x: &'a i32) -> &'a i32 {
    x
}
