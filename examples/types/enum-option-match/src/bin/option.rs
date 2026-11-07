//! `Option<u32>` is either `Some(age)` or `None`. There is no age without the variant.
//! Uncomment the addition to get error[E0369]: cannot add `{integer}` to `Option<u32>`.

fn find_age(name: &str) -> Option<u32> {
    if name == "Ferris" { Some(7) } else { None }
}

fn main() {
    let age = find_age("Ferris");
    // let next = age + 1;
    match age {
        Some(a) => println!("next year: {}", a + 1),
        None => println!("unknown"),
    }
}
