//! Two shared references read the same string; the owner keeps using it too.

fn main() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
    println!("{s}");
}
