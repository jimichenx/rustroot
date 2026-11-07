//! `parse` may fail, and the failure carries a reason.
//! The caller receives a `Result` and must look at both variants.

fn main() {
    for text in ["42", "forty-two"] {
        match text.parse::<i32>() {
            Ok(n) => println!("{text}: {n}"),
            Err(e) => println!("{text}: {e}"),
        }
    }
}
