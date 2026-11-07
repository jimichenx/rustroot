//! Without a trait bound, an unknown `T` only supports moving, borrowing,
//! and dropping. Uncomment `a + b` to get error[E0369]: cannot add `T` to `T`.

fn keep_second<T>(_first: T, second: T) -> T {
    // Moving `second` out and dropping `_first` works for any `T`:
    second
}

// fn add_values<T>(a: T, b: T) -> T {
//     a + b // error[E0369]: cannot add `T` to `T`
// }

fn main() {
    let n = keep_second(10, 20);
    let s = keep_second(String::from("old"), String::from("new"));
    println!("n = {n}, s = {s}");
}
