// Uncomment the marked line to trigger error[E0308].
fn main() {
    let mut n: i32 = 7;
    println!("{n}");
    n = 8;
    // n = String::from("hello");
    println!("{n}");
}
