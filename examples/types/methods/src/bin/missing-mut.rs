//! `push` takes `&mut self`. The bridge can only lend `&mut v` if `v` is a `mut` binding.
//! Uncomment the push to get error[E0596]: cannot borrow `v` as mutable,
//! as it is not declared as mutable.

fn main() {
    let v: Vec<i32> = Vec::new();
    // v.push(1);
    println!("{}", v.len());
}
