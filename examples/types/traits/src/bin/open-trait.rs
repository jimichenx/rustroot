//! An open contract. `Triangle` joins later without touching `Area` or `total`.

trait Area {
    fn area(&self) -> f64;
}

struct Circle(f64);
struct Square(f64);
struct Triangle(f64, f64);

impl Area for Circle {
    fn area(&self) -> f64 { 3.14 * self.0 * self.0 }
}
impl Area for Square {
    fn area(&self) -> f64 { self.0 * self.0 }
}
impl Area for Triangle {
    fn area(&self) -> f64 { self.0 * self.1 / 2.0 }
}

fn total(a: &impl Area, b: &impl Area) -> f64 {
    a.area() + b.area()
}

fn main() {
    println!("{}", total(&Circle(1.0), &Square(2.0)));
    println!("{}", total(&Square(2.0), &Triangle(3.0, 4.0)));
}
