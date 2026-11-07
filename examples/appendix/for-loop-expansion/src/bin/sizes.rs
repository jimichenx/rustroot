//! Sizes of the three iterator machines on a 64-bit platform.

use std::mem::size_of;

fn main() {
    println!("Iter<i32>     {}", size_of::<std::slice::Iter<'_, i32>>());
    println!("IterMut<i32>  {}", size_of::<std::slice::IterMut<'_, i32>>());
    println!("IntoIter<i32> {}", size_of::<std::vec::IntoIter<i32>>());
    println!("&[i32]        {}", size_of::<&[i32]>());
    println!("Vec<i32>      {}", size_of::<Vec<i32>>());
}
