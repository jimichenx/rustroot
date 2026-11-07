//! The shared borrow held by the iterator spans the whole loop.
//! Uncomment the marked line to get error[E0502].

fn main() {
    let mut v = vec![1, 2, 3];
    for x in &v {
        // v.push(*x); // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
        println!("{x}");
    }
    v.push(4); // fine here: the loop's borrow ended with the loop
    println!("{v:?}");
}
