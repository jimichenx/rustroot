//! The `?` operator works on both `Result<T, E>` and `Option<T>`:
//! - On `Ok(v)` / `Some(v)`, it unwraps `v` and continues.
//! - On `Err(e)` / `None`, it returns early from the current function.

use std::num::ParseIntError;

fn sum_two(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

struct Profile {
    city: Option<String>,
}

struct User {
    profile: Option<Profile>,
}

fn user_city(user: &User) -> Option<&str> {
    // Chain `?` across nested `Option`s without nested `match` blocks:
    let city = user.profile.as_ref()?.city.as_deref()?;
    Some(city)
}

fn main() {
    println!("{:?}", sum_two("1", "2"));
    println!("{:?}", sum_two("1", "two"));

    let u = User {
        profile: Some(Profile {
            city: Some(String::from("Shanghai")),
        }),
    };
    println!("city = {:?}", user_city(&u));
}
