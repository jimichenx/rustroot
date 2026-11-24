// Expect: E0716
fn main() {
    let text = String::from("hello");
    let existing = &text;
    let extended = &String::from("temp");
    // let bad = String::from("temp").as_str();
    println!("{existing} {extended}");
    // println!("{bad}");
}
