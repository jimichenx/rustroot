//! Defining a custom type constructor `Pair<T>`.
//! Uncomment `let _bad: Pair = ...` to get error[E0107]:
//! missing generics for struct `Pair`.

use std::mem::size_of;

struct Pair<T> {
    first: T,
    second: T,
}

fn main() {
    let bytes = Pair { first: 1u8, second: 2u8 };
    let words = Pair { first: 1u64, second: 2u64 };
    let texts = Pair {
        first: String::from("rust"),
        second: String::from("root"),
    };

    println!("Pair<u8>     size = {} bytes", size_of::<Pair<u8>>());
    println!("Pair<u64>    size = {} bytes", size_of::<Pair<u64>>());
    println!("Pair<String> size = {} bytes", size_of::<Pair<String>>());
    println!("bytes.first = {}, words.second = {}, texts.first = {}", bytes.first, words.second, texts.first);

    // `Pair` without `<T>` is not a concrete type entity yet:
    // let _bad: Pair = Pair { first: 1, second: 2 };
}
