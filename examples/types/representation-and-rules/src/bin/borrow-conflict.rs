// Expect: E0502
fn main() {
    // ANCHOR: disjoint
    let mut s = String::from("hello");
    let r: &String = &s;
    println!("{r}");
    let m: &mut String = &mut s;
    m.push_str(", world");
    println!("{m}");
    // ANCHOR_END: disjoint

    // ANCHOR: overlap
    let mut s = String::from("hello");
    let r: &String = &s;
    let m: &mut String = &mut s;
    m.push_str(", world");
    // println!("{r}");
    println!("{m}");
    // ANCHOR_END: overlap
}
