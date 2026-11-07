use std::any::Any;

trait Shape: Any {
    fn area(&self) -> f64;

    // The trait hands out a `&dyn Any`; without this exit there is no downcast.
    fn as_any(&self) -> &dyn Any;
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.r * self.r
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn describe(shape: &dyn Shape) {
    // Runtime type check: `None` when the concrete type is not `Rect`.
    match shape.as_any().downcast_ref::<Rect>() {
        Some(rect) => println!("rect {} x {} = {}", rect.w, rect.h, rect.area()),
        None => println!("not a rect, area = {}", shape.area()),
    }
}

fn main() {
    let rect = Rect { w: 3.0, h: 4.0 };
    let circle = Circle { r: 1.0 };

    describe(&rect);
    describe(&circle);
}
