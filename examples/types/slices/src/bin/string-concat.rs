// Uncomment the marked line to trigger error[E0382].
fn main() {
    let mut message = String::from("hello");
    message.push_str(" world");
    message.push('!');
    println!("mutated: {message}");

    let s1 = String::from("Rust");
    let s2 = String::from("Root");

    // `format!` borrows both strings without moving either.
    let formatted = format!("{s1}-{s2}");
    println!("formatted: {formatted} (s1 is still valid: {s1})");

    // `+` calls `fn add(self, s: &str) -> String`:
    // it moves `s1`, reuses its heap buffer, and borrows `s2`.
    let combined = s1 + " " + &s2;
    println!("combined: {combined} (s2 is still valid: {s2})");

    // println!("s1 after +: {s1}");
}
