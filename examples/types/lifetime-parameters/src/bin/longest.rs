//! From the signature alone, the caller cannot tell whether the result
//! borrows from `a` or from `b`.
//! Swap the two `fn` lines to get error[E0106]: missing lifetime specifier.

// fn longest(a: &str, b: &str) -> &str {
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let x = String::from("crab");
    let y = String::from("ferris");
    println!("{}", longest(&x, &y));
}
