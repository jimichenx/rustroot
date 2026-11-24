use std::ops::{Index, IndexMut};

fn main() {
    let mut values = vec![10, 20, 30];
    let first: i32 = *values.index(0);
    *values.index_mut(1) = 99;
    println!("first = {first}, values = {values:?}");
}
