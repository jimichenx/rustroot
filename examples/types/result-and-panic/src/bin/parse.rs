fn main() {
    for text in ["42", "forty-two"] {
        match text.parse::<i32>() {
            Ok(n) => println!("{text}: {n}"),
            Err(e) => println!("{text}: {e}"),
        }
    }
}
