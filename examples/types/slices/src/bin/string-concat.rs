// Expect: E0382
fn main() {
    let mut message = String::from("hello");
    message.push_str(" world");
    message.push('!');
    println!("mutated: {message}");

    let s1 = String::from("Rust");
    let s2 = String::from("Root");

    let formatted = format!("{s1}-{s2}");
    println!("formatted: {formatted} (s1 is still valid: {s1})");

    let combined = s1 + " " + &s2;
    println!("combined: {combined} (s2 is still valid: {s2})");

    // println!("s1 after +: {s1}");
}
