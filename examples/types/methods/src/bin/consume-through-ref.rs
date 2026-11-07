//! `into_bytes` takes `self`. Through a shared reference there is no value to hand over.
//! Uncomment the call to get error[E0507]: cannot move out of `*r`
//! which is behind a shared reference.

fn main() {
    let s = String::from("hi");
    let r = &s;
    println!("{}", r.len()); // `&self` methods work through `r`
    // let bytes = r.into_bytes();
    let bytes = s.into_bytes(); // the owner can hand it over
    println!("{bytes:?}");
}
