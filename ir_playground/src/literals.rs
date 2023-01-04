struct s {
    i: i32,
    u: u8,
}

enum e {
    C1(i32),
    C2(s),
}

fn main() {
    // let si = s { i: 42, u: 42 };
    let ei = e::C2(s { i: 42, u: 42 });
}
