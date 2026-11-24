// Expect: E0507
fn main() {
    let text = String::from("hello");
    let r = &text;
    // let s = *r;
    println!("{r}");
}
