mod shapes {
    pub trait Area {
        fn area(&self) -> f64;
    }

    pub struct Rect {
        pub w: f64,
        pub h: f64,
    }
}

// The impl lives away from the type's home module.
impl shapes::Area for shapes::Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}

fn main() {
    let rect = shapes::Rect { w: 3.0, h: 4.0 };

    // Remove this `use` line and the call below fails with error[E0599].
    use shapes::Area;
    println!("{}", rect.area());
}
