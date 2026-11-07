//! Generic functions and `impl<T>` blocks, plus turbofish `::<T>` when
//! the type parameter only appears in the return position.
//! Uncomment `let _ambiguous = "42".parse().unwrap();` to get error[E0284]:
//! type annotations needed.

struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn into_swapped(self) -> Pair<T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

fn first_item<T>(pair: Pair<T>) -> T {
    pair.first
}

fn main() {
    // 1. Argument provides `Pair<i32>`, so `T = i32` is solved automatically:
    let p = Pair { first: 10, second: 20 }.into_swapped();
    let head = first_item(p);

    // 2. Return-only generic parameter in `str::parse<F>()`:
    // let _ambiguous = "42".parse().unwrap(); // error[E0284]
    let by_binding: i32 = "42".parse().unwrap();
    let by_turbofish = "42".parse::<i64>().unwrap();

    println!("head = {head}, by_binding = {by_binding}, by_turbofish = {by_turbofish}");
}
