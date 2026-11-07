//! A struct that borrows its text lives no longer than the text's owner.
//! Uncomment `let moved = novel;` to get error[E0505]: cannot move out of
//! `novel` because it is borrowed.

struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first = novel.split('.').next().unwrap_or("");
    let e = Excerpt { part: first };
    // let moved = novel;
    println!("{}", e.part);
}
