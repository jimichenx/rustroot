// Uncomment the marked line to trigger error[E0277].
fn main() {
    let s = String::from("Rust语言");
    println!("byte length (.len()): {}", s.len());
    println!("char count (.chars()): {}", s.chars().count());

    // let first = s[0];
    let prefix: &str = &s[0..4];
    let suffix: &str = &s[4..10];
    println!("slices: {prefix} | {suffix}");

    print!("chars: ");
    for c in s.chars() {
        print!("[{c}] ");
    }
    println!();
}
