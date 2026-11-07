//! The fourth piece: no other borrow may be alive when the bridge lends `&mut v`.
//! Uncomment the push to get error[E0502]: cannot borrow `v` as mutable
//! because it is also borrowed as immutable.

#[allow(unused_mut)] // `mut` is needed once the push is uncommented
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    // v.push(4);
    println!("{first}");
}
