struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 1, y: 2 };
    point.x = 10;

    let y = &mut point.y;
    *y = 20;

    println!("{}, {}", point.x, point.y);
}
