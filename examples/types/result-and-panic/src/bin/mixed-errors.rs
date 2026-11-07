//! `?` converts the error into the function's error type with `From`.
//! `std::io::Error` has no conversion into `ParseIntError`.
//! Uncomment the `read_to_string` line to get error[E0277]:
//! `?` couldn't convert the error to `ParseIntError`.

use std::num::ParseIntError;

fn read_number(input: &str) -> Result<i32, ParseIntError> {
    // let input = std::fs::read_to_string(input)?;
    let n = input.trim().parse::<i32>()?;
    Ok(n)
}

fn main() {
    println!("{:?}", read_number(" 42 "));
}
