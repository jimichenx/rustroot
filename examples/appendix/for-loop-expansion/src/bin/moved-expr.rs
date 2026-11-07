//! The expression after `in` is moved into `into_iter` once, before the loop.
//! Uncomment the marked line to get error[E0382].

fn main() {
    let v = vec![1, 2, 3];
    for x in v {
        println!("{x}");
    }
    // println!("{v:?}"); // error[E0382]: borrow of moved value: `v`
}
