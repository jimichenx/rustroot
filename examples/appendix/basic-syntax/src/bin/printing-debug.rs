#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let values = [1, 2, 3];
    let point = Point { x: 4, y: 5 };

    println!("{values:?}");
    println!("{point:#?}");
    println!("({}, {})", point.x, point.y);
}
