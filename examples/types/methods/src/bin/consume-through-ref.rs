// Expect: E0507
fn main() {
    let s = String::from("hi");
    let r = &s;
    println!("{}", r.len());
    // let bytes = r.into_bytes();
    let bytes = s.into_bytes();
    println!("{bytes:?}");
}
