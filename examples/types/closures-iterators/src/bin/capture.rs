//! A closure is an anonymous `struct` synthesized by the compiler:
//! its size equals the sum of its captured fields, and its call traits
//! (`Fn`, `FnMut`, `FnOnce`) mirror `&self`, `&mut self`, and `self`.
//! Uncomment `let _again = consume();` to observe `error[E0382]`.

use std::mem::size_of_val;

fn main() {
    let empty = |x: i32| x + 1;
    let a = 10i32;
    let b = 20i32;
    let borrow_two = |x: i32| x + a + b;

    println!(
        "empty closure = {}B, borrow_two (&i32, &i32) = {}B",
        size_of_val(&empty),
        size_of_val(&borrow_two),
    );
    println!("empty(5) = {}, borrow_two(5) = {}", empty(5), borrow_two(5));

    // 1. `Fn` (&self): only reads captured `text`
    let text = String::from("Ferris");
    let read_len = || text.len();
    println!("Fn called twice: {}, {}", read_len(), read_len());

    // 2. `FnMut` (&mut self): mutates captured `count` (requires `let mut inc`)
    let mut count = 0;
    let mut inc = || {
        count += 1;
        count
    };
    println!("FnMut called twice: {}, {}", inc(), inc());

    // 3. `FnOnce` (self): moves `text` out of the closure's anonymous struct
    let consume = || text;
    println!("consume closure (holds String) = {}B", size_of_val(&consume));
    let owned: String = consume();
    println!("FnOnce returned: {owned}");

    // Uncomment to observe error[E0382]: use of moved value: `consume`
    // let _again = consume();
}
