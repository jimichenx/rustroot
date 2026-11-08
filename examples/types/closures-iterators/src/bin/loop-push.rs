//! The iterator `Iter<'a, i32>` holds a shared borrow of `v` across the entire loop,
//! and its size on 64-bit platforms is 16 bytes (two machine words: current ptr + end ptr).
//! Uncomment `v.push(*x);` inside the loop to observe `error[E0502]`.

use std::mem::size_of;

fn main() {
    println!(
        "Vec<i32> = {}B, &[i32] = {}B, slice::Iter<i32> = {}B",
        size_of::<Vec<i32>>(),
        size_of::<&[i32]>(),
        size_of::<std::slice::Iter<'_, i32>>(),
    );

    let mut v = vec![1, 2, 3];

    for x in &v {
        println!("visiting {x}");
        // Uncomment to observe error[E0502]: cannot borrow `v` as mutable
        // because it is also borrowed as immutable
        // v.push(*x);
    }

    // Once the loop ends, `iter` goes out of scope and the shared borrow ends:
    v.push(4);
    println!("after loop: {v:?}");
}
