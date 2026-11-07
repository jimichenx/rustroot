//! One generic body, one compiled copy per concrete type.
//! Each copy knows its own `T`, so each prints a different size.

use std::mem::size_of;

fn size_of_one<T>() -> usize {
    size_of::<T>()
}

fn main() {
    println!("u8:     {}", size_of_one::<u8>());
    println!("i32:    {}", size_of_one::<i32>());
    println!("String: {}", size_of_one::<String>());
}
