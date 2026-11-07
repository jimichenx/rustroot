fn main() {
    let text = String::from("hello");
    let existing = &text;                 // borrows an existing place
    let extended = &String::from("temp"); // borrows a fresh temporary
    println!("{existing} {extended}");
}
