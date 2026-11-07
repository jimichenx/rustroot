//! Every variant needs an arm.
//! Comment out the `Green` arm to get error[E0004]: non-exhaustive patterns:
//! `Light::Green` not covered.

enum Light {
    Red,
    Yellow,
    Green,
}

fn seconds(light: Light) -> u32 {
    match light {
        Light::Red => 30,
        Light::Yellow => 3,
        Light::Green => 25,
    }
}

fn main() {
    println!("{}", seconds(Light::Red) + seconds(Light::Yellow) + seconds(Light::Green));
}
