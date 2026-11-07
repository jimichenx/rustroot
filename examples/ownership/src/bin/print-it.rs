//! Passing a String by value moves it into the function.
//! Uncomment the last line in `main` to get error[E0382].

fn print_it(msg: String) {
    println!("{msg}");
}

fn main() {
    let s = String::from("hello");
    print_it(s);
    // println!("{s}");
}
