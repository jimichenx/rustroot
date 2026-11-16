// Expect: E0382
fn main() {
    let s = String::from("hello");
    let t = s;
    // println!("{s}");
    println!("{t}");
}
