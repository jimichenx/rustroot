//! Inside a `&self` method the field is behind a shared reference.
//! Swap the two `match` lines to get error[E0507]: cannot move out of
//! `self.name` as enum variant `Some` which is behind a shared reference.

struct User {
    name: Option<String>,
}

impl User {
    fn name_len(&self) -> usize {
        // match self.name {
        match &self.name {
            Some(n) => n.len(), // `n` is `&String`
            None => 0,
        }
    }
}

fn main() {
    let u = User { name: Some(String::from("Ferris")) };
    println!("{}", u.name_len());
}
