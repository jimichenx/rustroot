use std::mem::size_of;

struct User {
    name: String,
    age: u64,
    active: bool,
}

struct Color(u8, u8, u8);
struct AlwaysEqual;

fn build_user(name: String, age: u64) -> User {
    User {
        name, // field init shorthand for `name: name`
        age,
        active: true,
    }
}

fn main() {
    let u1 = build_user(String::from("Alice"), 28);
    // Struct update syntax `..u1` moves or copies remaining fields from `u1`.
    let u2 = User {
        name: String::from("Bob"),
        ..u1
    };

    let red = Color(255, 0, 0);
    let _marker = AlwaysEqual;

    println!("u2: {} (age {}, active {})", u2.name, u2.age, u2.active);
    println!("rgb = ({}, {}, {})", red.0, red.1, red.2);
    println!(
        "sizes: User = {}B, Color = {}B, AlwaysEqual = {}B",
        size_of::<User>(),
        size_of::<Color>(),
        size_of::<AlwaysEqual>()
    );
}
