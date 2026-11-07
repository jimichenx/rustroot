//! Two `i32` fields, no heap. Without a `Copy` declaration the struct still moves.
//! Uncomment the last line to get error[E0382]: borrow of moved value: `a`.

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
