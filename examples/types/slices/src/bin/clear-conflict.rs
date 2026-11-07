//! A slice is a borrow, so the 1.6 rule applies to it unchanged.
//! Uncomment `s.clear()` to get error[E0502]:
//! cannot borrow `s` as mutable because it is also borrowed as immutable.

#[allow(unused_mut)] // `mut` is needed once `s.clear()` is uncommented
fn main() {
    let mut s = String::from("hello world");
    let word: &str = &s[..5];

    // s.clear(); // error[E0502]: `word` is still alive here

    println!("{word}");
}
