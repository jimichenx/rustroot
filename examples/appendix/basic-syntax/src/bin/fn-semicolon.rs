// Uncomment the marked line (and comment out `a + b`) to trigger error[E0308].
fn add(a: i32, b: i32) -> i32 {
    a + b
    // a + b;
}

fn main() {
    let sum = add(3, 4);
    println!("{sum}");
}
