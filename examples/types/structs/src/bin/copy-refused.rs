//! A field that must be freed makes `Copy` impossible.
//! Change `#[derive(Clone)]` to `#[derive(Clone, Copy)]` to get error[E0204]:
//! the trait `Copy` cannot be implemented for this type.

#[derive(Clone)]
struct Named {
    name: String,
}

fn main() {
    let a = Named { name: String::from("Ferris") };
    let b = a.clone(); // an explicit second copy is still allowed
    println!("{} {}", a.name, b.name);
}
