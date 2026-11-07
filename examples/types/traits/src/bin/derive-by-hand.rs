//! `#[derive(Clone, Copy)]` written out by hand.
//! `Copy` has no methods: implementing it is only a declaration.

struct Point {
    x: i32,
    y: i32,
}

impl Clone for Point {
    fn clone(&self) -> Point {
        *self
    }
}

impl Copy for Point {}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a;
    println!("{} {}", a.x, b.y);
}
