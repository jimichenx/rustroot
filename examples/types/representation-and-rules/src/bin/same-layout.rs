//! `String` and `Vec<u8>` have the same layout: pointer, length, capacity.
//! The difference is a rule: a `String` only accepts bytes that are valid UTF-8.

use std::mem::size_of;

fn main() {
    println!("String:  {}", size_of::<String>());
    println!("Vec<u8>: {}", size_of::<Vec<u8>>());

    let good = vec![104, 105]; // "hi"
    let bad = vec![0xff, 0xfe]; // not UTF-8

    println!("{}", String::from_utf8(good).is_ok()); // true
    println!("{}", String::from_utf8(bad).is_ok()); // false
}
