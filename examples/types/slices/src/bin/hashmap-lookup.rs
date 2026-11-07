// Uncomment the marked insert line to trigger error[E0502].
use std::collections::HashMap;

#[allow(unused_mut)]
fn main() {
    let mut scores = HashMap::from([
        (String::from("Blue"), 10),
        (String::from("Yellow"), 50),
    ]);

    let blue_score = scores.get("Blue");
    // scores.insert(String::from("Red"), 30);
    println!("Blue: {:?}, Green: {:?}", blue_score, scores.get("Green"));
}
