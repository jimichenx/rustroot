// Uncomment the marked line to trigger error[E0502].
fn main() {
    let mut s = String::from("hello");
    let r: &String = &s;
    let m: &mut String = &mut s;
    m.push_str(", world");
    // println!("{r}");
    println!("{m}");
}
