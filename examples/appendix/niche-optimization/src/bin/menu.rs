//! Does wrapping in Option cost bytes? Only when the inner type has no illegal value.

use std::mem::size_of;
use std::num::NonZeroU32;

#[allow(dead_code)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    println!("&i32        -> Option<&i32>       {} -> {}", size_of::<&i32>(), size_of::<Option<&i32>>());
    println!("Box<i32>    -> Option<Box<i32>>   {} -> {}", size_of::<Box<i32>>(), size_of::<Option<Box<i32>>>());
    println!("bool        -> Option<bool>       {} -> {}", size_of::<bool>(), size_of::<Option<bool>>());
    println!("char        -> Option<char>       {} -> {}", size_of::<char>(), size_of::<Option<char>>());
    println!("NonZeroU32  -> Option<NonZeroU32> {} -> {}", size_of::<NonZeroU32>(), size_of::<Option<NonZeroU32>>());
    println!("String      -> Option<String>     {} -> {}", size_of::<String>(), size_of::<Option<String>>());
    println!("Person      -> Option<Person>     {} -> {}", size_of::<Person>(), size_of::<Option<Person>>());
    println!("i32         -> Option<i32>        {} -> {}", size_of::<i32>(), size_of::<Option<i32>>());
}
