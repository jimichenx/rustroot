use std::num::ParseIntError;

// ANCHOR: sum_two
fn sum_two(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}
// ANCHOR_END: sum_two

struct Profile {
    city: Option<String>,
}

struct User {
    profile: Option<Profile>,
}

// ANCHOR: user_city
fn user_city(user: &User) -> Option<&str> {
    let city = user.profile.as_ref()?.city.as_deref()?;
    Some(city)
}
// ANCHOR_END: user_city

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
