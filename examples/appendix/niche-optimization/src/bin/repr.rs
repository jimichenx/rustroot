//! repr(Rust) finds a seat; explicit repr(C)/repr(u8) refuse the search and pay a tag.

use std::mem::size_of;

#[allow(dead_code)]
enum WithData {
    Int(i32),
    Text(String),
    Nothing,
}

#[repr(C)]
#[allow(dead_code)]
enum WithDataC {
    Int(i32),
    Text(String),
    Nothing,
}

#[repr(u8)]
#[allow(dead_code)]
enum WithDataU8 {
    Int(i32),
    Text(String),
    Nothing,
}

fn main() {
    println!("String        {}", size_of::<String>());
    println!("WithData      {}", size_of::<WithData>());
    println!("WithData repr(C)  {}", size_of::<WithDataC>());
    println!("WithData repr(u8) {}", size_of::<WithDataU8>());
}
