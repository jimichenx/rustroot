//! Implementing `From` gives `Into` via a blanket implementation, and allows
//! the `?` operator to convert underlying errors into `AppError` automatically.

use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    EmptyInput,
    BadNumber(ParseIntError),
}

// Implementing `From<ParseIntError> for AppError`:
impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::BadNumber(err)
    }
}

fn parse_positive_double(raw: &str) -> Result<u32, AppError> {
    if raw.is_empty() {
        return Err(AppError::EmptyInput);
    }
    // `raw.parse::<u32>()` returns `Result<u32, ParseIntError>`.
    // `?` expands to `return Err(From::from(e))`, converting it to `AppError`!
    let n = raw.parse::<u32>()?;
    Ok(n * 2)
}

fn main() {
    for raw in ["21", "", "abc"] {
        match parse_positive_double(raw) {
            Ok(val) => println!("{raw:?} -> Ok({val})"),
            Err(AppError::EmptyInput) => println!("{raw:?} -> Err(EmptyInput)"),
            Err(AppError::BadNumber(e)) => println!("{raw:?} -> Err(BadNumber: {e})"),
        }
    }
}
