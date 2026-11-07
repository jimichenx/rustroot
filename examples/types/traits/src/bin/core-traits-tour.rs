//! Core standard library traits in action: `#[derive(...)]` for `Debug`,
//! `Clone`, `Copy`, `PartialEq`, `Default`, plus manual `Display` and `Add`.

use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let origin = Vec2::default();
    let step = Vec2 { x: 3, y: 4 };
    let moved = origin + step; // calls `Add::add(origin, step)`

    println!("debug = {moved:?}, display = {moved}, eq = {}", moved == step);
    // Blanket `impl<T: Display> ToString for T` gives `.to_string()` for free:
    let text: String = moved.to_string();
    println!("to_string = {text}");
}
