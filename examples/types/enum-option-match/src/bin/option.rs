// Expect: E0369
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
