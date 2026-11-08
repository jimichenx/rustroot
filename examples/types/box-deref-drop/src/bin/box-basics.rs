use std::mem::size_of;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let stack_pt = Point { x: 10, y: 20 };
    let ref_pt: &Point = &stack_pt;
    let boxed_pt: Box<Point> = Box::new(Point { x: 30, y: 40 });

    println!(
        "sizes: Point = {}B, &Point = {}B, Box<Point> = {}B",
        size_of::<Point>(),
        size_of::<&Point>(),
        size_of::<Box<Point>>()
    );
    println!("ref_pt: ({}, {})", ref_pt.x, ref_pt.y);
    println!("boxed_pt: ({}, {})", boxed_pt.x, boxed_pt.y);

    let moved_box = boxed_pt;
    // println!("{}", boxed_pt.x); // error[E0382]: borrow of moved value: `boxed_pt`
    println!("moved_box: ({}, {})", moved_box.x, moved_box.y);
}
