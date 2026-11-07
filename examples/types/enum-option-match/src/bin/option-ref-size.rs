//! Compare `size_of` before and after wrapping in `Option<T>`.
//! `u32` has no invalid bit pattern, so `Option<u32>` adds a tag and pads to 8 bytes.
//! `&i32` and `String` contain non-null pointers: the zero address (niche) represents `None` for free!

use std::mem::size_of;

fn main() {
    println!(
        "u32: {}B -> Option<u32>: {}B",
        size_of::<u32>(),
        size_of::<Option<u32>>()
    );
    println!(
        "&i32: {}B -> Option<&i32>: {}B",
        size_of::<&i32>(),
        size_of::<Option<&i32>>()
    );
    println!(
        "String: {}B -> Option<String>: {}B",
        size_of::<String>(),
        size_of::<Option<String>>()
    );
}
