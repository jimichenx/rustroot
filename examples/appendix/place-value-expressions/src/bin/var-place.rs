//! Uncomment the marked line and compile to get error[E0384].

fn main() {
    let mut count = 10;
    count = 20;

    let fixed = 10;
    // fixed = 20; // error[E0384]: cannot assign twice to immutable variable `fixed`

    println!("{count} {fixed}");
}
