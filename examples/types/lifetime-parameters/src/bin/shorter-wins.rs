//! The result may borrow from either argument, so it lives no longer than
//! the shorter one.
//! Move the `println!` below the inner block's closing brace to get
//! error[E0597]: `y` does not live long enough.

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let x = String::from("crab");
    let result;
    {
        let y = String::from("ferris");
        result = longest(&x, &y);
        println!("{result}");
    }
}
