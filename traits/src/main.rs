#![allow(unused, dead_code)]

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/*fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}*/

struct Pair<T, U> {
    first : T,
    second : U
}

impl<T, U> Pair<T, U> 
where T: Copy, U: Copy {
    fn first(&self)-> T {
        self.first
    }
    
    fn second(&self)-> U {
        self.second
    }
    
    fn dummy(&self) -> bool {
        true
    }
}

trait Printable {
    fn print(&self);
}

struct Container<T> {
    field : T
}

/*impl<T> Container<T>
where T : std::fmt::Display
{
    fn print(&self) {
        println!("I am general: field={}", self.field);
    }
}*/

impl Container<i32> {
    fn print(&self) {
        println!("I am i32: field={}",self.field);
    }
}

impl Container<u32> {
    fn print(&self) {
        println!("I am u32: field={}",self.field);
    }
}

/*
impl<T> Printable for Container<T>
{
    fn print(&self) {
        println!("My field is not printable!")
    }
}
*/



fn main() {
/*    let x = Container{field: 32i32};
    x.print();
    
    let y = Container{field: "String"};
    y.print();
    
    let z = Container{field: Ok(42) };
    x.print();*/
    
    let a = Container{field: 1i32};
    a.print();
    
    let b = Container{field: 1u32};
    b.print();
}