// Expect: E0596
fn main() {
    let v: Vec<i32> = Vec::new();
    // v.push(1);
    println!("{}", v.len());
}
