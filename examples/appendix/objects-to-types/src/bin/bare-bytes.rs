use std::mem::size_of;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("Point ({}, {}): {} bytes", p.x, p.y, size_of::<Point>());
    println!("[Point; 100]:   {} bytes", size_of::<[Point; 100]>());
}
