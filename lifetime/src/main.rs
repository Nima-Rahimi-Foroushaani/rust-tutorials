fn main() {
    //    let r: &i32;
    //
    //    {
    //        let x = 5;
    //        r = &x;
    //    }
    //
    //    println!("r is {}", r);

    //    let string1 = String::from("long string is long");
    //    let result;
    //    {
    //        let string2 = String::from("xyz");
    //        result = longest(string1.as_str(), string2.as_str());
    //    }
    //    println!("The longest string is {}", result);

    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // let result = ident(result, string2);
    // println!("The longest string is {}", result);
    // There is no referring to string1 or string2 here but it seems their life also has extended because of life time of the "result"
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn ident<'a>(x: &'a str, _dummy: &str) -> &'a str {
//     x
// }

fn erroneous<'b, 'a: 'b>(i: &'a i32) -> &'b i32 {
    i
}
