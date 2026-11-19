// Expect: E0382
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a;
    println!("{} {}", b.x, b.y);
    // println!("{}", a.x);
}
