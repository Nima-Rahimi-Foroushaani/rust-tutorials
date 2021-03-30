struct Animal<'a> {
    fluffType: &'a str,
}

impl<'a> Animal<'a> {
    fn fluffAndColor(&self, color: &str) -> &str {
        self.fluffType
//        color
    }
}

//struct Object {
//    name: &str,
//}
// Why there are not elision rules for struct fields?

fn main() {
    println!("Hello, world!");
}
