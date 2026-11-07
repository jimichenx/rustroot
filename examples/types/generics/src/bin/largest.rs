//! A generic function has to say what `T` can do.
//! Swap the two `fn` lines to get error[E0369]: binary operation `>`
//! cannot be applied to type `T`.

// fn largest<T>(items: &[T]) -> T {
fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut max = items[0];
    for &item in items {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    println!("{}", largest(&[3, 7, 2]));
    println!("{}", largest(&[1.5, 0.5]));
    println!("{}", largest(&['a', 'z', 'm']));
}
