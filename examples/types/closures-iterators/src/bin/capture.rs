//! A closure captures each name in the lightest way its body needs.
//! Uncomment the second `take()` to get error[E0382]: use of moved value: `take`.

fn main() {
    let name = String::from("Ferris");
    let read = || println!("{name}"); // reads: borrows `&name`
    read();
    read();

    let mut count = 0;
    let mut inc = || count += 1; // writes: borrows `&mut count`
    inc();
    inc();
    println!("{count}");

    let take = || name; // returns `name`: moves it in
    let owned = take();
    // let again = take();
    println!("{owned}");
}
