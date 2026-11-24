// Expect: E0382
use std::mem::size_of_val;

fn main() {
    let empty = |x: i32| x + 1;
    let a = 10i32;
    let b = 20i32;
    let borrow_two = |x: i32| x + a + b;

    println!(
        "empty closure = {}B, borrow_two (&i32, &i32) = {}B",
        size_of_val(&empty),
        size_of_val(&borrow_two),
    );
    println!("empty(5) = {}, borrow_two(5) = {}", empty(5), borrow_two(5));

    let text = String::from("Ferris");
    let read_len = || text.len();
    println!("Fn called twice: {}, {}", read_len(), read_len());

    let mut count = 0;
    let mut inc = || {
        count += 1;
        count
    };
    println!("FnMut called twice: {}, {}", inc(), inc());

    let consume = || text;
    println!("consume closure (holds String) = {}B", size_of_val(&consume));
    let owned: String = consume();
    println!("FnOnce returned: {owned}");

    // let _again = consume();
}
