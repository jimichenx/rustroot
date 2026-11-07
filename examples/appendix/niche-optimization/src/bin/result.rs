//! Result gets seats too: a two-machine-word &str can host an Err in its len slot.

use std::mem::size_of;

fn main() {
    println!("&str                 {}", size_of::<&str>());
    println!("Result<&str, i32>    {}", size_of::<Result<&str, i32>>());
    println!("Result<i32, i32>     {}", size_of::<Result<i32, i32>>());
    println!("Result<bool, bool>   {}", size_of::<Result<bool, bool>>());
}
