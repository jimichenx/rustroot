fn main() {
    let s = String::from("hello");
    let r: &String = &s;
    println!("{s} {r}");
}
