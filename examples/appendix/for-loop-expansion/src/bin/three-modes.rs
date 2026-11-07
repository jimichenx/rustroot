//! Three loop modes over the same data: shared borrow, exclusive borrow, ownership.
//! Uncomment the marked line to get error[E0382].

fn main() {
    let mut v = vec![1, 2, 3];

    let mut sum = 0;
    for x in &v {
        let _: &i32 = x; // element arrives as a shared reference
        sum += *x;
    }
    println!("shared: {v:?}, sum = {sum}");

    for x in &mut v {
        let _: &mut i32 = x; // element arrives as an exclusive reference
        *x *= 10;
    }
    println!("exclusive: {v:?}");

    for x in v {
        let _: i32 = x; // element arrives by value
        println!("took {x}");
    }
    // println!("{v:?}"); // error[E0382]: borrow of moved value: `v`
}
