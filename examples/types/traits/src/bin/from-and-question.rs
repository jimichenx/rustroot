use std::num::ParseIntError;

// ANCHOR: app_error
#[derive(Debug)]
enum AppError {
    EmptyInput,
    BadNumber(ParseIntError),
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::BadNumber(err)
    }
}

fn parse_positive_double(raw: &str) -> Result<u32, AppError> {
    if raw.is_empty() {
        return Err(AppError::EmptyInput);
    }
    let n = raw.parse::<u32>()?;
    Ok(n * 2)
}
// ANCHOR_END: app_error

fn main() {
    for raw in ["21", "", "abc"] {
        match parse_positive_double(raw) {
            Ok(val) => println!("{raw:?} -> Ok({val})"),
            Err(AppError::EmptyInput) => println!("{raw:?} -> Err(EmptyInput)"),
            Err(AppError::BadNumber(e)) => println!("{raw:?} -> Err(BadNumber: {e})"),
        }
    }
}
