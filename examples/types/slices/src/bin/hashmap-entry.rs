use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("after or_insert: Blue = {:?}, Yellow = {:?}", scores.get("Blue"), scores.get("Yellow"));

    let text = "rust types rust ownership rust";
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("rust count: {:?}, types count: {:?}", counts.get("rust"), counts.get("types"));
}
