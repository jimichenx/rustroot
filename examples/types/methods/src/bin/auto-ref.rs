//! The method call inserts the borrow for you. All three lines call the same function.
//! `contains` lives on `str`; the call on a `String` reaches it through the bridge.

fn main() {
    let mut v = vec![1, 2, 3];
    Vec::push(&mut v, 4); // written out in full
    (&mut v).push(5); // borrow written by hand
    v.push(6); // borrow inserted by the compiler
    println!("{v:?}");

    let s = String::from("hello world");
    println!("{}", str::contains(&s, "wor"));
    println!("{}", s.contains("wor"));
}
