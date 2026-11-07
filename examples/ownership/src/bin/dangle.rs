//! A borrow cannot outlive its owner.
//! Uncomment the block in `main` to get error[E0597]: `x` does not live long enough.

fn main() {
    // let r = {
    //     let x = String::from("hello");
    //     &x
    // };
    // println!("{r}");
}
