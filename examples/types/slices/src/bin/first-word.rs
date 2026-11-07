//! The slice version: take a view, return a view.
//! `&String` coerces to `&str` at the call, so owners and literals both work.

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] == b' ' {
            return &s[..i];
        }
    }
    s
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);

    let lit = first_word("ferris crab");

    println!("{word} {lit}");
}
