//! `move` moves every captured name into the closure, so the closure
//! can outlive the frame that created it.
//! Remove `move` to get error[E0373]: closure may outlive the current function,
//! but it borrows `name`.
//! The closure only reads `name`, so it is still `Fn`: `call_twice` accepts it.

fn make_greeter(name: String) -> impl Fn() -> String {
    move || format!("hello, {name}")
}

fn call_twice(f: impl Fn() -> String) {
    println!("{}", f());
    println!("{}", f());
}

fn main() {
    let greet = make_greeter(String::from("Ferris"));
    call_twice(greet);
}
