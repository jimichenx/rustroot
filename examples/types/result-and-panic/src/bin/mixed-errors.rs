// Expect: E0277
use std::num::ParseIntError;

fn read_number(input: &str) -> Result<i32, ParseIntError> {
    // let input = std::fs::read_to_string(input)?;
    let n = input.trim().parse::<i32>()?;
    Ok(n)
}

fn main() {
    println!("{:?}", read_number(" 42 "));
}
