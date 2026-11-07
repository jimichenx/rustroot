//! `for` accepts anything with an `IntoIterator` implementation: ranges and
//! adapter chains arrive through the blanket impl for all iterators.
//! Uncomment the marked line to get error[E0277].

fn main() {
    let mut total = 0;
    for i in 0..5 {
        total += i;
    }
    println!("range: {total}");

    let words = ["alpha", "beta", "gamma"];
    for n in words.iter().map(|w| w.len()) {
        println!("len: {n}");
    }

    // for x in 5 {} // error[E0277]: `{integer}` is not an iterator
}
