//! Retroactive implementation vs. the Orphan Rule (coherence).
//! Implementing our `Shout` trait for standard library `String` works;
//! uncommenting `impl fmt::Display for Vec<i32>` triggers error[E0117].

trait Shout {
    fn shout(&self) -> String;
}

// Allowed: `Shout` is defined in our crate, even though `String` is in `std`.
impl Shout for String {
    fn shout(&self) -> String {
        format!("{}!!!", self.to_uppercase())
    }
}

// Forbidden by the Orphan Rule (error[E0117]): neither `Display` nor `Vec<T>`
// is defined in the current crate.
// impl std::fmt::Display for Vec<i32> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "vec")
//     }
// }

fn main() {
    let msg = String::from("rust");
    println!("{}", msg.shout());
}
