//! One field offers one seat: one payload-free variant fits, two do not.

use std::mem::size_of;

#[allow(dead_code)]
enum OneFree {
    A,
    P(&'static i32),
}

#[allow(dead_code)]
enum TwoNeed {
    A,
    B,
    P(&'static i32),
}

fn main() {
    println!("OneFree          {}", size_of::<OneFree>());
    println!("TwoNeed          {}", size_of::<TwoNeed>());
    println!("Option<&i32>     {}", size_of::<Option<&'static i32>>());
}
