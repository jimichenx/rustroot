//! Passing the error up by hand: every call unfolds into a `match`.

use std::num::ParseIntError;

fn sum_two(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = match a.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    let y = match b.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    Ok(x + y)
}

fn main() {
    println!("{:?}", sum_two("1", "2"));
    println!("{:?}", sum_two("1", "two"));
}
