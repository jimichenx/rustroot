//! The same struct with `Copy` declared. Assignment now copies.

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a;
    println!("{} {}", a.x, b.y); // both bindings are usable
}
