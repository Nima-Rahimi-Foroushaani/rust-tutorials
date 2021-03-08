#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn square(size: u32)-> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle { width: 50, height: 30 };
    
    println!("rectangle is {:#?}", rect1);
    println!("The area of rectangle is {}", rect1.area());
    println!("An Square: {:#?}", Rectangle::square(30));
}