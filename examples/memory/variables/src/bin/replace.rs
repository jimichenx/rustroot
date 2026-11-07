fn main() {
    let mut target = String::from("old");
    let source = String::from("new");
    println!("{target} {source}");
    target = source;
    println!("{target}");
}
