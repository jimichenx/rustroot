//! A binding cannot be reassigned unless it is declared with `mut`.
//! Remove `mut` to get error[E0384]: cannot assign twice to immutable variable.

fn main() {
    let original = 1;
    let mut total = original;
    total += 1;

    println!("{original} {total}");
}
