fn main() {
    let boxed = Box::new(String::from("hello"));
    let s = *boxed;
    println!("{s}");
}
