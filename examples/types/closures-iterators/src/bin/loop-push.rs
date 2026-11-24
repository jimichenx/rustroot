// Expect: E0502
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
        // v.push(*x);
    }

    v.push(4);
    println!("after loop: {v:?}");
}
