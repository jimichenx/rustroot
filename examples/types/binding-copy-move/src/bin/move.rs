//! Uncomment the next line and compile to get error[E0382].
//! After the move, `s` cannot be used until it is assigned a new value.

fn main() {
    let s = String::from("hello");
    let t = s;
    // println!("{s}");
    println!("{t}");
}
