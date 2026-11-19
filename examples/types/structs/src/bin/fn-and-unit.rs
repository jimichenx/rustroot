// Expect: E0308
use std::mem::size_of;

// ANCHOR: add_fn
fn add(a: i32, b: i32) -> i32 {
    a + b
    // a + b;
}
// ANCHOR_END: add_fn

// ANCHOR: min_max_fn
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b { (a, b) } else { (b, a) }
}
// ANCHOR_END: min_max_fn

// ANCHOR: unit_fn
fn log_ready(tag: &str) {
    println!("[{tag}] ready");
}
// ANCHOR_END: unit_fn

fn main() {
    let sum: i32 = add(3, 4);
    // ANCHOR: tuple_use
    let pair: (i32, i32) = min_max(9, 2);
    let (low, high) = pair;
    // ANCHOR_END: tuple_use
    let unit_val: () = log_ready("init");

    println!(
        "sum = {sum}, min = {low}, max = {high} (or {}), unit = {:?}, size of () = {}B",
        pair.1,
        unit_val,
        size_of::<()>()
    );
}
