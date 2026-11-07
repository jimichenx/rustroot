//! When the seat is contested or buys no byte, rustc pays for an explicit tag.

use std::mem::size_of;

#[allow(dead_code)]
enum Mixed {
    Nothing,
    Ptr(&'static i32),
    Int(i32),
}

#[allow(dead_code)]
enum TwoPtrs {
    A(&'static i32),
    B(&'static i32),
}

fn main() {
    println!("Mixed    {}", size_of::<Mixed>());
    println!("TwoPtrs  {}", size_of::<TwoPtrs>());
}
