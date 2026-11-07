use std::ops::IndexMut;

fn main() {
    let mut values = vec![1, 2, 3];
    values[0] = 10;
    *values.index_mut(1) = 20;
    println!("{values:?}");
}
