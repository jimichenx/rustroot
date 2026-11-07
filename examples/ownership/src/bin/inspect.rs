//! Passing a reference (`&String`) keeps the original variable valid after the call.

fn print_it(msg: &String) {
    println!("{msg}");
}

fn main() {
    let s = String::from("hello");
    print_it(&s);
    println!("{s}");
}
