// Uncomment the marked line (and comment out `a + b`) to trigger error[E0308].
use std::mem::size_of;

fn add(a: i32, b: i32) -> i32 {
    a + b
    // a + b;
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b { (a, b) } else { (b, a) }
}

fn log_ready(tag: &str) {
    println!("[{tag}] ready");
}

fn main() {
    let sum: i32 = add(3, 4);
    let pair: (i32, i32) = min_max(9, 2);
    let (low, high) = pair;
    let unit_val: () = log_ready("init");

    println!(
        "sum = {sum}, min = {low}, max = {high} (or {}), unit = {:?}, size of () = {}B",
        pair.1,
        unit_val,
        size_of::<()>()
    );
}
