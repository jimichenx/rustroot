//! A view is a value of two machine words: address + length.
//! Copying a view copies those two words, not the data.
//! `size_of::<T>()` is the standard library function reporting a type's
//! byte size on the current machine.

use std::mem::size_of;

fn main() {
    let s = String::from("hello");
    let a: &str = &s[..3];
    let b = a; // copies address + length, not the characters
    println!("{a} {b}"); // both bindings still work: a view is Copy

    println!("&[i32; 5]: {}", size_of::<&[i32; 5]>()); // 8: length in the type
    println!("&[i32]:    {}", size_of::<&[i32]>()); // 16: length in the value
    println!("Vec<i32>:  {}", size_of::<Vec<i32>>()); // 24: pointer + length + capacity
}
