//! One reference in, one reference out: the compiler writes the annotation.
//! The two functions below have the same signature.
//! A string literal borrows from the program itself: `&'static str`.

fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or(s)
}

fn first_word_explicit<'a>(s: &'a str) -> &'a str {
    s.split(' ').next().unwrap_or(s)
}

fn main() {
    let s = String::from("hello world");
    let greeting: &'static str = "hi there";
    println!("{} {}", first_word(&s), first_word_explicit(greeting));
}
