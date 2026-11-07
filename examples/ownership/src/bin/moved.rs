//! Uncomment the commented line below and compile to get error[E0382].
//! After `let t = s`, `s` cannot be used again.

fn main() {
    let s = String::from("hello");
    let t = s;
    // println!("{s}");
    println!("{t}");
}
