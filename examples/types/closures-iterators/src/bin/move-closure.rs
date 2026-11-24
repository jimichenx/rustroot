// Expect: E0373
use std::mem::size_of_val;

fn make_greeter(prefix: String) -> impl Fn(&str) -> String {
    move |target: &str| format!("{prefix}, {target}")
}

fn call_twice<F>(f: F)
where
    F: Fn(&str) -> String,
{
    println!("{}", f("Rust"));
    println!("{}", f("Root"));
}

fn main() {
    let greet = make_greeter(String::from("Hello"));
    println!("size of move-Fn closure = {}B", size_of_val(&greet));
    call_twice(greet);
}
