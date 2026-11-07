//! A trait can be implemented for a type written by someone else.
//! `String` comes from the standard library; `Shout` is ours.

trait Shout {
    fn shout(&self) -> String;
}

impl Shout for String {
    fn shout(&self) -> String {
        self.to_uppercase() + "!"
    }
}

fn main() {
    let s = String::from("hello");
    println!("{}", s.shout());
}
