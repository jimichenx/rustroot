// Uncomment the println! line to trigger error[E0382].
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    let team = String::from("Blue");
    let initial_score = 10;

    scores.insert(team, initial_score);
    // println!("team: {team}");
    println!("initial_score is still valid: {initial_score}");

    let old = scores.insert(String::from("Blue"), 25);
    println!("replaced old value: {:?}, current map: {:?}", old, scores);
}
