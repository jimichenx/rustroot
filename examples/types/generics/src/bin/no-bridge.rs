//! The method-call bridge does not exist at a generic parameter.
//! `r.summary()` works on `&String`, but `&String` itself does not implement `Summary`.
//! Uncomment `show(r)` to get error[E0277]: the trait bound
//! `&String: Summary` is not satisfied.

trait Summary {
    fn summary(&self) -> usize;
}

impl Summary for String {
    fn summary(&self) -> usize {
        self.len()
    }
}

fn show<T: Summary>(x: T) -> usize {
    x.summary()
}

fn show_ref<T: Summary>(x: &T) -> usize {
    x.summary()
}

fn main() {
    let s = String::from("hello");
    let r = &s;
    println!("{}", r.summary()); // the bridge finds `String::summary`
    // println!("{}", show(r));
    println!("{}", show_ref(r)); // the reference is written in the signature
    println!("{}", show(s)); // or hand over the owned value
}
