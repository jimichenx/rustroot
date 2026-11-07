//! Clone makes an explicit copy, so both strings remain usable.

fn main() {
    let first = String::from("hello");
    let second = first.clone();

    println!("{first}");
    println!("{second}");
}
