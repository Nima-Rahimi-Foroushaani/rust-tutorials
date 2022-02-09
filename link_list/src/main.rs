use std::option::Option;

// #[derive(Debug)]
struct LinkList<'a, T> {
    data: T,
    next: Option<&'a mut LinkList<'a, T>>,
}

fn main() {
    let mut l: LinkList<i32> = LinkList {
        data: 42,
        next: None,
    };
    let l1: LinkList<i32> = LinkList {
        data: 43,
        next: Some(&mut l),
    };
}
