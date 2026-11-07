// Uncomment the marked line to trigger error[E0596].
fn main() {
    let s = String::from("hello");
    // s.push_str("!");
    println!("{s}");
}
