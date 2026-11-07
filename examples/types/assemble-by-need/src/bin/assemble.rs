//! Three requirements, three signatures assembled from the four questions.

use std::num::ParseIntError;

// Read a piece of text, produce a new one.
fn shout(text: &str) -> String {
    text.to_uppercase()
}

// A setting that may be absent.
fn port(setting: Option<String>) -> u16 {
    match setting {
        Some(s) => s.parse().unwrap_or(80),
        None => 80,
    }
}

// Parse every number in a text, or report the first failure.
fn parse_all(text: &str) -> Result<Vec<i32>, ParseIntError> {
    let mut out = Vec::new();
    for word in text.split_whitespace() {
        out.push(word.parse()?);
    }
    Ok(out)
}

fn main() {
    println!("{}", shout("hello"));
    println!("{} {}", port(Some(String::from("8080"))), port(None));
    println!("{:?} {:?}", parse_all("1 2 3"), parse_all("1 two"));
}
