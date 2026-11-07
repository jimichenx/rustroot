struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn show(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

fn main() {
    let mut point = Point { x: 1, y: 2 };
    let reference = &mut point;

    reference.x = 10;
    reference.show();
}
