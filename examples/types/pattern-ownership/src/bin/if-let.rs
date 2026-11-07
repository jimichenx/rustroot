//! When only one variant matters.
//! `if let`: the other variants do nothing.
//! `let else`: the other variants must leave the function.

fn greet(name: Option<&str>) -> String {
    let Some(n) = name else {
        return String::from("hello, stranger");
    };
    format!("hello, {n}")
}

fn main() {
    let name = Some("Ferris");
    if let Some(n) = name {
        println!("found {n}");
    }
    println!("{}", greet(None));
    println!("{}", greet(name));
}
