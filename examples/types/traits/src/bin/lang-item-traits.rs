//! Compiler-hooked standard library traits (lang items):
//! `Copy`/`Clone`, `PartialEq`, and `Add`.

use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2 {
    x: i32,
    y: i32,
}

// Implementing the `Add` lang item hooks into the `+` operator:
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

trait Label {
    fn name(&self) -> &'static str {
        "Vec2(Label)"
    }
}

impl Vec2 {
    fn name(&self) -> &'static str {
        "Vec2(inherent)"
    }
}

impl Label for Vec2 {}

fn main() {
    let a = Vec2 { x: 1, y: 2 };
    let b = a; // `Copy` hook: bitwise copy, `a` remains valid
    let sum = a + b; // `Add` hook: desugars to `<Vec2 as Add>::add(a, b)`
    println!("sum = {sum:?}, equal? {}", a == b); // `PartialEq` hook: `==`
    println!("inherent = {}, trait = {}", a.name(), <Vec2 as Label>::name(&a));
}
