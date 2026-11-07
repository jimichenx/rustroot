//! The same view for sequences: `&[T]` borrows from arrays and `Vec<T>` alike.

fn sum(slice: &[i32]) -> i32 {
    let mut total = 0;
    for i in 0..slice.len() {
        total += slice[i];
    }
    total
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let a = [1, 2, 3, 4, 5];

    println!("{}", sum(&v[1..])); // borrows part of the Vec
    println!("{}", sum(&a[..])); // borrows the whole array
}
