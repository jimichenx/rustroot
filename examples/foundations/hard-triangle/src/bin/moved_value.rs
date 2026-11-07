//! Lesson 1 · Code experiment placeholder
//!
//! Can the four commented-out lines below compile? Why or why not?
//!
//! ```text
//! let s = String::from("root");
//! let t = s;
//! println!("{s}");   // error[E0382]: borrow of moved value: `s`
//! ```
//!
//! Uncomment the lines, run `cargo run --bin moved-value`, and inspect the
//! compiler's decision.

fn main() {
    let s = String::from("root");
    let t = s;
    println!("{t}");
}
