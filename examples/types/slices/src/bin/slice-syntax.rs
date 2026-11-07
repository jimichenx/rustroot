//! Range syntax creates a slice: a view into part of the owner.
//! The type of `&s[0..5]` is `&str`, not `&String`.

fn main() {
    let s = String::from("hello world");

    let head: &str = &s[0..5];
    let tail: &str = &s[6..];
    let whole: &str = &s[..];

    let literal: &str = "hello world"; // a literal is already a slice

    println!("{head} {tail} {whole} {literal}");
}
