// Uncomment the marked line to trigger error[E0382].
fn main() {
    let mut s = String::from("hello");
    let t = s;
    // println!("{s}");
    s = String::from("world");
    println!("{s} {t}");
}
