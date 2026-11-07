//! Uncomment the marked line and compile to get error[E0507].

fn main() {
    let text = String::from("hello");
    let r = &text;
    // let s = *r; // error[E0507]: cannot move out of `*r` which is behind a shared reference
    println!("{r}");
}
