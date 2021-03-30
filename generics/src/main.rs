#[derive(Debug)]
struct Rectangle<T> {
    l: T,
    w: T
}
//There is no Specialization but specific types may have specific extra functions

//impl Rectangle<f32> {
//    fn w(&self) -> &f32 {
//        println!("This is f32 specific implementation");
//        &self.w
//    }
//}

impl<T> Rectangle<T> {
    fn w(&self) -> &T {
        println!("This is the default implementation");
        &self.w
    }
    
    fn add<U>(&self, r: Rectangle<U>) -> Rectangle<U> {
        //needs some extra Trait bound
        //Rectangle{w: self.w + r.w, l: self.l + r.l}
        r
    }
}

impl Rectangle<f32> {
    fn diameter(&self) -> f32 {
        (self.w.powi(2) + self.l.powi(2)).sqrt()
    }
}

fn main() {
    
    let r = Rectangle{
        w: 10,
        l: 20,
    };
    
    println!("The width of {:?} is {}", r, r.w());
    // diameter is defined only for Rectangle<f32>
    // let dr = r.diameter();
    
    let rf = Rectangle{w:5.4, l: 9.2};
    println!("The diameter of rf is {}", rf.diameter());
}
