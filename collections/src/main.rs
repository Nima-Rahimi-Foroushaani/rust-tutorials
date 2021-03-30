fn main() {
    let v: Vec<i32> = Vec::new();
    
    let vs = vec![String::from("Hello"), String::from("borrow"), String::from("checker!")];
    
    //let hello = vs[1]; // move is not allowed out of indexing
    
    let mut v = vec![100, 32, 57];
    let r = &mut v[1];
    *r += 10;
    for i in &mut v {
        *i += 50;
    }

    let hello = "Здравствуйте";
    //let s = &hello[0..3]; // panics
    
    
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
}
