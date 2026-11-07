trait Shape {
    fn area(&self) -> f64;
}

struct Rect {
    w: f64,
    h: f64,
}

struct Circle {
    r: f64,
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.r * self.r
    }
}

// Static dispatch: one compiled copy per concrete type.
fn print_area<T: Shape>(shape: &T) {
    println!("static: {}", shape.area());
}

// Dynamic dispatch: one copy; each call goes through a vtable.
fn print_area_dyn(shape: &dyn Shape) {
    println!("dynamic: {}", shape.area());
}

fn main() {
    let rect = Rect { w: 3.0, h: 4.0 };
    let circle = Circle { r: 1.0 };

    print_area(&rect);
    print_area_dyn(&rect);
    print_area_dyn(&circle);
}
