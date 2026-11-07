//! Bounds stack side by side: the argument must be cloneable and printable.
//! Remove `+ Debug` to get error[E0277]: `impl Clone` doesn't implement `Debug`.

use std::fmt::Debug;

fn keep_and_show(item: &(impl Clone + Debug)) {
    let kept = item.clone();
    println!("{kept:?}");
}

fn main() {
    keep_and_show(&vec![1, 2, 3]);
    keep_and_show(&String::from("hi"));
}
