//! Ergonomic helpers on `Option<T>`:
//! - `unwrap_or`: provide a fallback value when `None`.
//! - `as_ref`: borrow `&Option<String>` as `Option<&String>` without moving.
//! - `take`: swap out `Some(v)` from a `&mut Option<T>` and leave `None` in place.

struct User {
    name: Option<String>,
    nickname: Option<String>,
}

fn main() {
    let mut u = User {
        name: Some(String::from("Ferris")),
        nickname: None,
    };

    // 1. `as_ref()` + `unwrap_or()`: read without moving `u.nickname`
    let display = u.nickname.as_ref().map(|s| s.as_str()).unwrap_or("anonymous");
    println!("display = {display}");

    // 2. `as_ref()`: turn `&Option<String>` into `Option<&String>`
    let peek: Option<&String> = u.name.as_ref();
    println!("peek = {peek:?}");

    // 3. `take()`: move the `String` out through `&mut`, leaving `None` behind
    let taken: Option<String> = u.name.take();
    println!("taken = {taken:?}, remaining u.name = {:?}", u.name);
}
