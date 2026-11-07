//! The same lookup, two contracts.
//! `v[i]` promises the index is valid; if not, the program panics.
//! `v.get(i)` admits it may be missing and returns `Option`.
//! Uncomment the indexing line to panic at run time: index out of bounds.

fn main() {
    let v = vec![10, 20, 30];
    let i = 10;

    println!("{:?}", v.get(i)); // None
    // println!("{}", v[i]);

    let n: i32 = "42".parse().expect("a literal we wrote ourselves");
    println!("{n}");
}
