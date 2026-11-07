// Uncomment the marked line to trigger a runtime panic.
fn main() {
    let s = String::from("语言");
    let valid_slice = &s[0..3];
    println!("first char slice: {valid_slice}");

    // Safe slicing in production: returns Option<&str> instead of panicking.
    println!("safe get 0..3: {:?}", s.get(0..3));
    println!("safe get 0..1: {:?}", s.get(0..1));

    // let bad_slice = &s[0..1];
}
