//! Lesson 1.6 on an iterator: the loop holds a shared borrow of `v`
//! for its whole length.
//! Uncomment the push to get error[E0502]: cannot borrow `v` as mutable
//! because it is also borrowed as immutable.

#[allow(unused_mut)] // `mut` is needed once the push is uncommented
fn main() {
    let mut v = vec![1, 2, 3];
    for x in &v {
        println!("{x}");
        // v.push(*x);
    }
}
