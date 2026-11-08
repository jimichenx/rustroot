//! Orthogonal axes: how a closure captures (`&` vs `move`) is independent of
//! how it is called (`Fn` vs `FnMut` vs `FnOnce`).
//! A `move` closure that only reads its captured `String` still implements `Fn`
//! and can be called multiple times.
//! Removing `move` inside `make_greeter` triggers `error[E0373]`.

use std::mem::size_of_val;

fn make_greeter(prefix: String) -> impl Fn(&str) -> String {
    // `move` forces `prefix` (24B String) into the anonymous struct by value
    // so the closure can outlive `make_greeter`'s stack frame.
    // Because the body only reads `prefix`, the closure still implements `Fn` (&self).
    move |target: &str| format!("{prefix}, {target}")
}

fn call_twice<F>(f: F)
where
    F: Fn(&str) -> String,
{
    println!("{}", f("Rust"));
    println!("{}", f("Root"));
}

fn main() {
    let greet = make_greeter(String::from("Hello"));
    println!("size of move-Fn closure = {}B", size_of_val(&greet));
    call_twice(greet);
}
