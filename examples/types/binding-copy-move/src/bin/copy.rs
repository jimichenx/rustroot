//! Copy values remain usable after assignment.

fn main() {
    let first = 42;
    let second = first;

    println!("{first}");
    println!("{second}");
}
