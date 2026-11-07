//! After a move the binding holds no value, like a binding never assigned.
//! Assigning a new value makes it usable again.
//! Uncomment the first `println!` to get error[E0382]: borrow of moved value: `s`.

fn main() {
    let mut s = String::from("hello");
    let t = s;
    // println!("{s}");
    s = String::from("world");
    println!("{t} {s}");
}
