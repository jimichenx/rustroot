//! A closed set of cases. Adding a triangle means editing the enum
//! and every `match` over it.

enum Shape {
    Circle(f64),
    Square(f64),
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Square(a) => a * a,
    }
}

fn main() {
    println!("{}", area(&Shape::Circle(1.0)) + area(&Shape::Square(2.0)));
}
