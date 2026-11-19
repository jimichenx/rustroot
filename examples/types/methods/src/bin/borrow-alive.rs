// Expect: E0502
#[allow(unused_mut)]
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    // v.push(4);
    println!("{first}");
}
