//! A borrow ends at its last use, not at the closing brace.
//!
//! `r` is last used by the first `println!`, so the borrow of `s` ends there
//! and the `push` below is legal. Moving that `println!` below the `push`
//! extends the borrow past the write, and the program no longer compiles:
//! error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable.

fn main() {
    let mut s = String::from("hello");

    let r = &s;
    println!("{r}");

    s.push('!');
    println!("{s}");
}
